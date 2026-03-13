# ReturnOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**created_by** | [**models::User**](User.md) |  | [readonly]
**creation_date** | Option<[**String**](String.md)> |  | [optional]
**issue_date** | Option<[**String**](String.md)> | Date order was issued | [optional]
**start_date** | Option<[**String**](String.md)> | Scheduled start date for this order | [optional]
**target_date** | Option<[**String**](String.md)> | Expected date for order delivery. Order will be overdue after this date. | [optional]
**description** | Option<**String**> | Order description (optional) | [optional]
**line_items** | Option<**i32**> |  | [optional][readonly]
**completed_lines** | Option<**i32**> |  | [optional][readonly]
**link** | Option<**String**> | Link to external page | [optional]
**project_code** | Option<**i32**> | Select project code for this order | [optional]
**reference** | **String** |  | 
**responsible** | Option<**i32**> | User or group responsible for this order | [optional]
**contact** | Option<**i32**> | Point of contact for this order | [optional]
**address** | Option<**i32**> | Company address for this order | [optional]
**status** | **i32** |  | [readonly]
**status_text** | **String** |  | [readonly]
**status_custom_key** | Option<**i32**> | Additional status information for this item | [optional][readonly]
**barcode_hash** | **String** |  | [readonly]
**overdue** | Option<**bool**> |  | [optional][readonly]
**duplicate** | Option<[**models::DuplicateOrder**](DuplicateOrder.md)> | Specify options for duplicating this order | [optional]
**address_detail** | Option<[**models::AddressBrief**](AddressBrief.md)> |  | [optional][readonly]
**contact_detail** | Option<[**models::Contact**](Contact.md)> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]
**project_code_label** | Option<**String**> |  | [optional][readonly]
**responsible_detail** | Option<[**models::Owner**](Owner.md)> |  | [optional][readonly]
**complete_date** | Option<[**String**](String.md)> | Date order was completed | [optional]
**customer** | Option<**i32**> | Company from which items are being returned | [optional]
**customer_reference** | Option<**String**> | Customer order reference code | [optional]
**order_currency** | Option<**String**> | Currency for this order (leave blank to use company default) | [optional]
**total_price** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


