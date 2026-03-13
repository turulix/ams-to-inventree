# PatchedSupplierPriceBreak

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**part** | Option<**i32**> |  | [optional]
**quantity** | Option<**f64**> |  | [optional]
**price** | Option<**String**> |  | [optional]
**price_currency** | Option<**String**> | Select currency from available options  * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**supplier** | Option<**i32**> |  | [optional][readonly]
**updated** | Option<**String**> | Timestamp of last update | [optional][readonly]
**supplier_detail** | Option<[**models::CompanyBrief**](CompanyBrief.md)> |  | [optional][readonly]
**part_detail** | Option<[**models::SupplierPart**](SupplierPart.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


