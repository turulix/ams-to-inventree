# PatchedReturnOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**created_by** | Option<[**models::User**](User.md)> |  | [optional][readonly]
**creation_date** | Option<[**String**](String.md)> |  | [optional]
**issue_date** | Option<[**String**](String.md)> | Date order was issued | [optional]
**start_date** | Option<[**String**](String.md)> | Scheduled start date for this order | [optional]
**target_date** | Option<[**String**](String.md)> | Expected date for order delivery. Order will be overdue after this date. | [optional]
**description** | Option<**String**> | Order description (optional) | [optional]
**line_items** | Option<**i32**> |  | [optional][readonly]
**completed_lines** | Option<**i32**> |  | [optional][readonly]
**link** | Option<**String**> | Link to external page | [optional]
**project_code** | Option<**i32**> | Select project code for this order | [optional]
**reference** | Option<**String**> |  | [optional]
**responsible** | Option<**i32**> | User or group responsible for this order | [optional]
**contact** | Option<**i32**> | Point of contact for this order | [optional]
**address** | Option<**i32**> | Company address for this order | [optional]
**status** | Option<**i32**> |  | [optional][readonly]
**status_text** | Option<**String**> |  | [optional][readonly]
**status_custom_key** | Option<**i32**> | Additional status information for this item  * `10` - Pending * `20` - In Progress * `25` - On Hold * `30` - Complete * `40` - Cancelled  Additional custom status keys may be retrieved from the corresponding 'status_retrieve' call. | [optional][readonly]
**notes** | Option<**String**> | Markdown notes (optional) | [optional]
**barcode_hash** | Option<**String**> |  | [optional][readonly]
**overdue** | Option<**bool**> |  | [optional][readonly]
**duplicate** | Option<[**models::DuplicateOrder**](DuplicateOrder.md)> | Specify options for duplicating this order | [optional]
**address_detail** | Option<[**models::AddressBrief**](AddressBrief.md)> |  | [optional][readonly]
**contact_detail** | Option<[**models::Contact**](Contact.md)> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]
**project_code_label** | Option<**String**> |  | [optional][readonly]
**responsible_detail** | Option<[**models::Owner**](Owner.md)> |  | [optional][readonly]
**parameters** | Option<[**Vec<models::Parameter>**](Parameter.md)> |  | [optional][readonly]
**complete_date** | Option<[**String**](String.md)> | Date order was completed | [optional]
**customer** | Option<**i32**> | Company from which items are being returned | [optional]
**customer_detail** | Option<[**models::CompanyBrief**](CompanyBrief.md)> |  | [optional][readonly]
**customer_reference** | Option<**String**> | Customer order reference code | [optional]
**order_currency** | Option<**String**> | Currency for this order (leave blank to use company default)  * `` - --------- * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**total_price** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


