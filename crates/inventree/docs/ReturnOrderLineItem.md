# ReturnOrderLineItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**link** | Option<**String**> | Link to external page | [optional]
**notes** | Option<**String**> | Line item notes | [optional]
**order** | **i32** | Return Order | 
**project_code** | Option<**i32**> | Select project code for this order | [optional]
**quantity** | **f64** | Quantity to return | 
**reference** | Option<**String**> | Line item reference | [optional]
**target_date** | Option<[**String**](String.md)> |  | [optional]
**order_detail** | Option<[**models::ReturnOrder**](ReturnOrder.md)> |  | [optional][readonly]
**project_code_label** | Option<**String**> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]
**item** | **i32** | Select item to return from customer | 
**received_date** | Option<[**String**](String.md)> | The date this return item was received | [optional]
**outcome** | Option<[**models::OutcomeEnum**](OutcomeEnum.md)> | Outcome for this line item  * `10` - Pending * `20` - Return * `30` - Repair * `40` - Replace * `50` - Refund * `60` - Reject | [optional]
**price** | Option<**String**> |  | [optional]
**price_currency** | Option<**String**> | Line price currency  * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**item_detail** | Option<[**models::StockItem**](StockItem.md)> |  | [optional][readonly]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


