# PatchedBomItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**part** | Option<**i32**> | Select the parent assembly | [optional]
**sub_part** | Option<**i32**> | Select the component part | [optional]
**reference** | Option<**String**> | BOM item reference | [optional]
**quantity** | Option<**f64**> |  | [optional]
**allow_variants** | Option<**bool**> | Stock items for variant parts can be used for this BOM item | [optional]
**inherited** | Option<**bool**> | This BOM item is inherited by BOMs for variant parts | [optional]
**optional** | Option<**bool**> | This BOM item is optional | [optional]
**consumable** | Option<**bool**> | This BOM item is consumable (it is not tracked in build orders) | [optional]
**setup_quantity** | Option<**f64**> |  | [optional]
**attrition** | Option<**f64**> |  | [optional]
**rounding_multiple** | Option<**f64**> |  | [optional]
**note** | Option<**String**> | BOM item notes | [optional]
**pk** | Option<**i32**> |  | [optional][readonly]
**pricing_min** | Option<**String**> |  | [optional][readonly]
**pricing_max** | Option<**String**> |  | [optional][readonly]
**pricing_min_total** | Option<**String**> |  | [optional][readonly]
**pricing_max_total** | Option<**String**> |  | [optional][readonly]
**pricing_updated** | Option<**String**> |  | [optional][readonly]
**substitutes** | Option<[**Vec<models::BomItemSubstitute>**](BomItemSubstitute.md)> |  | [optional][readonly]
**validated** | Option<**bool**> | This BOM item has been validated | [optional]
**available_stock** | Option<**f64**> |  | [optional][readonly]
**available_substitute_stock** | Option<**f64**> |  | [optional][readonly]
**available_variant_stock** | Option<**f64**> |  | [optional][readonly]
**external_stock** | Option<**f64**> |  | [optional][readonly]
**on_order** | Option<**f64**> |  | [optional][readonly]
**building** | Option<**f64**> |  | [optional][readonly]
**can_build** | Option<**f64**> |  | [optional][readonly]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**sub_part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**category_detail** | Option<[**models::Category**](Category.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


