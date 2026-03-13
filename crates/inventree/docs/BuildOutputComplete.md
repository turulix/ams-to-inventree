# BuildOutputComplete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**outputs** | [**Vec<models::BuildOutputQuantity>**](BuildOutputQuantity.md) |  | 
**location** | **i32** | Location for completed build outputs | 
**status_custom_key** | Option<**i32**> | Stock item status code  * `10` - OK * `50` - Attention needed * `55` - Damaged * `60` - Destroyed * `65` - Rejected * `70` - Lost * `75` - Quarantined * `85` - Returned  Additional custom status keys may be retrieved from the 'stock_status_retrieve' call. | [optional][default to 10]
**accept_incomplete_allocation** | Option<**bool**> | Complete outputs if stock has not been fully allocated | [optional][default to false]
**notes** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


