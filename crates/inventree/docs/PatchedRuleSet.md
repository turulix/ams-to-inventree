# PatchedRuleSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**name** | Option<[**models::NameEnum**](NameEnum.md)> | Permission set  * `admin` - Admin * `part_category` - Part Categories * `part` - Parts * `stock_location` - Stock Locations * `stock` - Stock Items * `build` - Build Orders * `purchase_order` - Purchase Orders * `sales_order` - Sales Orders * `return_order` - Return Orders | [optional][readonly]
**label** | Option<**String**> | Return the translated label for this ruleset. | [optional][readonly]
**group** | Option<**i32**> | Group | [optional][readonly]
**can_view** | Option<**bool**> | Permission to view items | [optional]
**can_add** | Option<**bool**> | Permission to add items | [optional]
**can_change** | Option<**bool**> | Permissions to edit items | [optional]
**can_delete** | Option<**bool**> | Permission to delete items | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


