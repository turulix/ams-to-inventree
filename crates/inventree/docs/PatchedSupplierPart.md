# PatchedSupplierPart

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available** | Option<**f64**> |  | [optional]
**availability_updated** | Option<**String**> | Date of last update of availability data | [optional][readonly]
**description** | Option<**String**> | Supplier part description | [optional]
**in_stock** | Option<**f64**> |  | [optional][readonly]
**on_order** | Option<**f64**> |  | [optional][readonly]
**link** | Option<**String**> | URL for external supplier part link | [optional]
**active** | Option<**bool**> | Is this supplier part active? | [optional]
**manufacturer_detail** | Option<[**models::CompanyBrief**](CompanyBrief.md)> |  | [optional][readonly]
**manufacturer_part** | Option<**i32**> | Select manufacturer part | [optional]
**manufacturer_part_detail** | Option<[**models::ManufacturerPart**](ManufacturerPart.md)> |  | [optional][readonly]
**mpn** | Option<**String**> |  | [optional][readonly]
**note** | Option<**String**> | Notes | [optional]
**pk** | Option<**i32**> |  | [optional][readonly]
**barcode_hash** | Option<**String**> | Unique hash of barcode data | [optional][readonly]
**packaging** | Option<**String**> | Part packaging | [optional]
**pack_quantity** | Option<**String**> | Total quantity supplied in a single pack. Leave empty for single items. | [optional]
**pack_quantity_native** | Option<**f64**> |  | [optional][readonly]
**part** | Option<**i32**> | Select part | [optional]
**pretty_name** | Option<**String**> |  | [optional][readonly]
**sku** | Option<**String**> | Supplier stock keeping unit | [optional]
**supplier** | Option<**i32**> |  | [optional]
**supplier_detail** | Option<[**models::CompanyBrief**](CompanyBrief.md)> |  | [optional][readonly]
**updated** | Option<**String**> |  | [optional][readonly]
**notes** | Option<**String**> | Markdown notes (optional) | [optional]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**tags** | Option<**Vec<String>**> |  | [optional]
**price_breaks** | Option<[**Vec<models::SupplierPriceBreakBrief>**](SupplierPriceBreakBrief.md)> |  | [optional][readonly]
**parameters** | Option<[**Vec<models::Parameter>**](Parameter.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


