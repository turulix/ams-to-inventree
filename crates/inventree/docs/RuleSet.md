# RuleSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | [**models::NameEnum**](NameEnum.md) | Permission set  * `admin` - Admin * `part_category` - Part Categories * `part` - Parts * `stock_location` - Stock Locations * `stock` - Stock Items * `build` - Build Orders * `purchase_order` - Purchase Orders * `sales_order` - Sales Orders * `return_order` - Return Orders | [readonly]
**label** | **String** | Return the translated label for this ruleset. | [readonly]
**group** | **i32** | Group | [readonly]
**can_view** | Option<**bool**> | Permission to view items | [optional]
**can_add** | Option<**bool**> | Permission to add items | [optional]
**can_change** | Option<**bool**> | Permissions to edit items | [optional]
**can_delete** | Option<**bool**> | Permission to delete items | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


