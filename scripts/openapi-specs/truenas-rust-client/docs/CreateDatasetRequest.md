# CreateDatasetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the new dataset | 
**pool** | **String** | Pool to create dataset in | 
**r#type** | Option<**Type**> |  (enum: FILESYSTEM, VOLUME) | [optional][default to Filesystem]
**volsize** | Option<**i32**> | Volume size for VOLUME type | [optional]
**compression** | Option<**String**> |  | [optional]
**deduplication** | Option<**String**> |  | [optional]
**quota** | Option<**i32**> |  | [optional]
**refquota** | Option<**i32**> |  | [optional]
**readonly** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


