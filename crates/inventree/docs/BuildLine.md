# BuildLine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**build** | **i32** | Build object | [readonly]
**bom_item** | **i32** |  | [readonly]
**quantity** | **f64** |  | 
**consumed** | **f64** |  | 
**allocations** | Option<[**Vec<models::BuildItem>**](BuildItem.md)> |  | [optional][readonly]
**part** | **i32** |  | [readonly]
**build_reference** | **String** |  | [readonly]
**reference** | **String** |  | [readonly]
**consumable** | **bool** |  | [readonly]
**optional** | **bool** |  | [readonly]
**testable** | **bool** |  | [readonly]
**trackable** | **bool** |  | [readonly]
**inherited** | **bool** |  | [readonly]
**allow_variants** | **bool** |  | [readonly]
**allocated** | **f64** |  | [readonly]
**in_production** | **f64** |  | [readonly]
**scheduled_to_build** | **f64** |  | [readonly]
**on_order** | **f64** |  | [readonly]
**available_stock** | **f64** |  | [readonly]
**available_substitute_stock** | **f64** |  | [readonly]
**available_variant_stock** | **f64** |  | [readonly]
**external_stock** | **f64** |  | [readonly]
**bom_item_detail** | Option<[**models::BomItem**](BomItem.md)> |  | [optional][readonly]
**assembly_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**category_detail** | Option<[**models::Category**](Category.md)> |  | [optional][readonly]
**build_detail** | Option<[**models::Build**](Build.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


