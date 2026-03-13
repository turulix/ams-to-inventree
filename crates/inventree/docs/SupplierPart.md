# SupplierPart

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Supplier part description | [optional]
**in_stock** | Option<**f64**> |  | [optional][readonly]
**link** | Option<**String**> | URL for external supplier part link | [optional]
**active** | Option<**bool**> | Is this supplier part active? | [optional]
**manufacturer_part** | Option<**i32**> | Select manufacturer part | [optional]
**mpn** | Option<**String**> |  | [optional][readonly]
**note** | Option<**String**> | Notes | [optional]
**pk** | **i32** |  | [readonly]
**barcode_hash** | **String** | Unique hash of barcode data | [readonly]
**packaging** | Option<**String**> | Part packaging | [optional]
**pack_quantity** | Option<**String**> | Total quantity supplied in a single pack. Leave empty for single items. | [optional]
**pack_quantity_native** | **f64** |  | [readonly]
**part** | **i32** | Select part | 
**sku** | **String** | Supplier stock keeping unit | 
**supplier** | **i32** |  | 
**updated** | Option<**String**> |  | [optional][readonly]
**notes** | Option<**String**> | Markdown notes (optional) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


