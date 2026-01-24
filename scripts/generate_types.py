#!/usr/bin/env python3
"""
Generate Rust types from OpenAPI specification.
This is a fallback when openapi-generator-cli is not available.
"""

import json
import sys
import re
from pathlib import Path
from typing import Dict, List, Any, Optional

def to_snake_case(name: str) -> str:
    """Convert CamelCase to snake_case."""
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()

def to_pascal_case(name: str) -> str:
    """Convert snake_case to PascalCase."""
    parts = name.replace('-', '_').split('_')
    return ''.join(p.title().replace('_', '') for p in parts if p)

def sanitize_name(name: str) -> str:
    """Sanitize a string to be a valid Rust identifier."""
    # Handle special characters and prefixes
    name = name.replace('-', '_').replace('.', '_').replace('/', '_')
    # Remove leading numbers if any
    if name and name[0].isdigit():
        name = f'_{name}'
    return name

def get_rust_type(schema: Dict[str, Any], schemas: Dict[str, Any]) -> str:
    """Convert OpenAPI schema to Rust type."""
    if schema is None:
        return "serde_json::Value"

    schema_type = schema.get('type', 'object')
    ref = schema.get('$ref', '')

    if ref:
        # Handle $ref
        ref_name = ref.split('/')[-1]
        return to_pascal_case(ref_name)

    if schema_type == 'array':
        items = schema.get('items', {})
        item_type = get_rust_type(items, schemas)
        return f"Vec<{item_type}>"

    if schema_type == 'object':
        # Check for additionalProperties
        if 'additionalProperties' in schema:
            prop_type = get_rust_type(schema['additionalProperties'], schemas)
            return f"HashMap<String, {prop_type}>"
        return "serde_json::Value"

    # Primitive types
    type_map = {
        'integer': 'i64',
        'number': 'f64',
        'string': 'String',
        'boolean': 'bool',
        'null': '()',
    }
    format_map = {
        'int32': 'i32',
        'int64': 'i64',
        'float': 'f32',
        'double': 'f64',
        'date': 'chrono::NaiveDate',
        'date-time': 'chrono::DateTime<chrono::Utc>',
        'byte': 'Vec<u8>',
        'binary': 'Vec<u8>',
        'password': 'String',
        'email': 'String',
        'uri': 'String',
        'uuid': 'uuid::Uuid',
    }

    if 'format' in schema:
        return format_map.get(schema['format'], type_map.get(schema_type, 'serde_json::Value'))

    return type_map.get(schema_type, 'serde_json::Value')

def generate_struct(name: str, schema: Dict[str, Any], schemas: Dict[str, Any], indent: str = "    ") -> str:
    """Generate Rust struct from schema."""
    lines = []

    # Get description for doc comment
    description = schema.get('description', '')
    if description:
        lines.append(f"{indent}/// {description}")

    lines.append(f"{indent}#[derive(Debug, Clone, Serialize, Deserialize, Default)]")
    lines.append(f"{indent}#[serde(rename_all = \"camelCase\")]")
    lines.append(f"{indent}pub struct {name} {{")

    required_fields = schema.get('required', [])

    for prop_name, prop_schema in schema.get('properties', {}).items():
        prop_type = get_rust_type(prop_schema, schemas)
        is_required = prop_name in required_fields

        # Add doc comment
        prop_desc = prop_schema.get('description', '')
        if prop_desc:
            lines.append(f"{indent}    /// {prop_desc}")

        # Add serde attributes
        serde_attrs = []
        if prop_name != to_snake_case(prop_name):
            serde_attrs.append(f"rename = \"{prop_name}\"")

        # Handle Option for optional fields
        if not is_required:
            prop_type = f"Option<{prop_type}>"

        serde_attr = ""
        if serde_attrs:
            serde_attr = f"    #[serde({', '.join(serde_attrs)})]"

        lines.append(f"{indent}    {serde_attr}")
        lines.append(f"{indent}    pub {to_snake_case(prop_name)}: {prop_type},")

    lines.append(f"{indent}}}")
    lines.append("")

    return "\n".join(lines)

def generate_enum(name: str, schema: Dict[str, Any], indent: str = "    ") -> str:
    """Generate Rust enum from schema."""
    lines = []

    description = schema.get('description', '')
    if description:
        lines.append(f"{indent}/// {description}")

    enum_type = schema.get('type', 'string')
    default_variant = schema.get('default', '')

    lines.append(f"{indent}#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]")
    lines.append(f"{indent}#[serde(rename_all = \"kebab-case\")]")
    lines.append(f"{indent}pub enum {name} {{")

    if enum_type == 'string':
        for i, (value, _) in enumerate(schema.get('enum', [])):
            # Sanitize variant name
            variant_name = to_pascal_case(value)
            if variant_name[0].isdigit():
                variant_name = f"Variant{variant_name}"

            if value == default_variant:
                lines.append(f"{indent}    #[default]")

            lines.append(f"{indent}    {variant_name},")
    else:
        # Numeric enum
        for i, value in enumerate(schema.get('enum', [])):
            lines.append(f"{indent}    V{i} = {value},")

    lines.append(f"{indent}}}")
    lines.append("")

    return "\n".join(lines)

def generate_client(api_def: Dict[str, Any], spec_file: Path) -> str:
    """Generate the complete Rust client module."""
    lines = []

    # Header
    lines.append("// Auto-generated TrueNAS API client")
    lines.append("// Generated from: " + str(spec_file))
    lines.append("")
    lines.append("use crate::error::TrueNasResult;")
    lines.append("use serde::{{Deserialize, Serialize}};")
    lines.append("use std::collections::HashMap;")
    lines.append("")

    # Process schemas
    schemas = api_def.get('components', {}).get('schemas', api_def.get('definitions', {}))
    path_items = api_def.get('paths', {})

    lines.append("// ==================== Types ====================")
    lines.append("")

    # Generate structs
    for name, schema in schemas.items():
        pascal_name = to_pascal_case(name)
        schema_type = schema.get('type', 'object')

        if schema_type == 'object' and 'properties' in schema:
            lines.append(generate_struct(pascal_name, schema, schemas))
        elif schema_type == 'string' and 'enum' in schema:
            lines.append(generate_enum(pascal_name, schema))
        elif 'oneOf' in schema:
            # Handle oneOf as enum
            lines.append(f"{'    '}// oneOf: {name}")
            lines.append(f"{'    '}#[derive(Debug, Clone, Serialize, Deserialize)]")
            lines.append(f"{'    '}#[serde(tag = \"type\", content = \"data\")]")
            lines.append(f"{'    '}pub enum {pascal_name} {{")
            lines.append(f"{'        '}Unknown(serde_json::Value),")
            lines.append(f"{'    }}}}")
            lines.append("")

    # Generate client methods
    lines.append("// ==================== Client ====================")
    lines.append("")
    lines.append("/// Generated API client for TrueNAS")
    lines.append("pub struct TrueNasApiClient {")
    lines.append("    base_url: String,")
    lines.append("    headers: HashMap<String, String>,")
    lines.append("}")
    lines.append("")
    lines.append("impl TrueNasApiClient {")
    lines.append("    /// Create a new client")
    lines.append("    pub fn new(base_url: &str, api_key: &str) -> Self {")
    lines.append("        let mut headers = HashMap::new();")
    lines.append("        headers.insert(\"Authorization\".to_string(), format!(\"Bearer {}\", api_key));")
    lines.append("        headers.insert(\"Content-Type\".to_string(), \"application/json\".to_string());")
    lines.append("")
    lines.append("        Self {")
    lines.append("            base_url: base_url.trim_end_matches('/').to_string(),")
    lines.append("            headers,")
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Make a GET request")
    lines.append("    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> TrueNasResult<T> {")
    lines.append("        let url = format!(\"{}/{}\", self.base_url, path.trim_start_matches('/'));")
    lines.append("        // Implementation would use reqwest")
    lines.append("        unimplemented!()")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Make a POST request")
    lines.append("    pub async fn post<T: for<'de> Deserialize<'de>, B: Serialize>("/)
    lines.append("        &self,")
    lines.append("        path: &str,")
    lines.append("        body: &B,")
    lines.append("    ) -> TrueNasResult<T> {")
    lines.append("        let url = format!(\"{}/{}\", self.base_url, path.trim_start_matches('/'));")
    lines.append("        // Implementation would use reqwest")
    lines.append("        unimplemented!()")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    return "\n".join(lines)

def main():
    if len(sys.argv) < 3:
        print("Usage: generate_types.py <spec-file> <output-file>")
        sys.exit(1)

    spec_file = Path(sys.argv[1])
    output_file = Path(sys.argv[2])

    print(f"Loading OpenAPI spec from: {spec_file}")

    with open(spec_file, 'r') as f:
        api_def = json.load(f)

    print(f"Generating Rust client...")

    client_code = generate_client(api_def, spec_file)

    with open(output_file, 'w') as f:
        f.write(client_code)

    print(f"Generated: {output_file}")

    # Print summary
    schemas = api_def.get('components', {}).get('schemas', api_def.get('definitions', {}))
    paths = api_def.get('paths', {})

    print(f"\nSummary:")
    print(f"  Schemas: {len(schemas)}")
    print(f"  Paths: {len(paths)}")

if __name__ == '__main__':
    main()
