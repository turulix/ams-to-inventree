# ManufacturerPart

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**part** | **i32** | Select part | 
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**pretty_name** | Option<**String**> |  | [optional][readonly]
**manufacturer** | **i32** |  | 
**manufacturer_detail** | Option<[**models::CompanyBrief**](CompanyBrief.md)> |  | [optional][readonly]
**description** | Option<**String**> | Manufacturer part description | [optional]
**mpn** | Option<**String**> | Manufacturer Part Number | [optional]
**link** | Option<**String**> | URL for external manufacturer part link | [optional]
**barcode_hash** | Option<**String**> | Unique hash of barcode data | [optional]
**notes** | Option<**String**> | Markdown notes (optional) | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**parameters** | Option<[**Vec<models::Parameter>**](Parameter.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


