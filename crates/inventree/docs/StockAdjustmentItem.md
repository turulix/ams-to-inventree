# StockAdjustmentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** | StockItem primary key value | 
**quantity** | **String** |  | 
**batch** | Option<**String**> | Batch code for this stock item | [optional]
**status** | Option<**i32**> | Stock item status code  * `None` - No Change * `10` - OK * `50` - Attention needed * `55` - Damaged * `60` - Destroyed * `65` - Rejected * `70` - Lost * `75` - Quarantined * `85` - Returned  Additional custom status keys may be retrieved from the 'stock_status_retrieve' call. | [optional]
**packaging** | Option<**String**> | Packaging this stock item is stored in | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


