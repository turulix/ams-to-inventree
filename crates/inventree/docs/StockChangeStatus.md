# StockChangeStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**items** | **Vec<i32>** | Select stock items to change status | 
**status** | Option<**i32**> | Stock item status code  * `10` - OK * `50` - Attention needed * `55` - Damaged * `60` - Destroyed * `65` - Rejected * `70` - Lost * `75` - Quarantined * `85` - Returned  Additional custom status keys may be retrieved from the 'stock_status_retrieve' call. | [optional][default to 10]
**note** | Option<**String**> | Add transaction note (optional) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


