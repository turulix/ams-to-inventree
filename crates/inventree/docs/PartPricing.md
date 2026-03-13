# PartPricing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | Option<**String**> |  | [optional][readonly]
**updated** | Option<**String**> |  | [optional][readonly]
**scheduled_for_update** | **bool** |  | [readonly]
**bom_cost_min** | Option<**String**> |  | [optional][readonly]
**bom_cost_max** | Option<**String**> |  | [optional][readonly]
**purchase_cost_min** | Option<**String**> |  | [optional][readonly]
**purchase_cost_max** | Option<**String**> |  | [optional][readonly]
**internal_cost_min** | Option<**String**> |  | [optional][readonly]
**internal_cost_max** | Option<**String**> |  | [optional][readonly]
**supplier_price_min** | Option<**String**> |  | [optional][readonly]
**supplier_price_max** | Option<**String**> |  | [optional][readonly]
**variant_cost_min** | Option<**String**> |  | [optional][readonly]
**variant_cost_max** | Option<**String**> |  | [optional][readonly]
**override_min** | Option<**String**> | Override calculated value for minimum price | [optional]
**override_min_currency** | Option<**String**> | Select currency from available options  * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**override_max** | Option<**String**> | Override calculated value for maximum price | [optional]
**override_max_currency** | Option<**String**> | Select currency from available options  * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**overall_min** | Option<**String**> |  | [optional][readonly]
**overall_max** | Option<**String**> |  | [optional][readonly]
**sale_price_min** | Option<**String**> |  | [optional][readonly]
**sale_price_max** | Option<**String**> |  | [optional][readonly]
**sale_history_min** | Option<**String**> |  | [optional][readonly]
**sale_history_max** | Option<**String**> |  | [optional][readonly]
**update** | Option<**bool**> | Update pricing for this part | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


