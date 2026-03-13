# LocationTree

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** | Name | 
**parent** | Option<**i32**> |  | [optional]
**icon** | **String** | Get the current icon used for this location.  The icon field on this model takes precedences over the possibly assigned stock location type | [readonly]
**structural** | Option<**bool**> | Stock items may not be directly located into a structural stock locations, but may be located to child locations. | [optional]
**sublocations** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


