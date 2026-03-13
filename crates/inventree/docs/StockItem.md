# StockItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**part** | **i32** | Base Part | 
**quantity** | **f64** |  | 
**serial** | Option<**String**> | Serial number for this item | [optional]
**batch** | Option<**String**> | Batch code for this stock item | [optional]
**location** | Option<**i32**> | Where is this stock item located? | [optional]
**belongs_to** | Option<**i32**> | Is this item installed in another item? | [optional]
**build** | Option<**i32**> | Build for this stock item | [optional]
**consumed_by** | Option<**i32**> | Build order which consumed this stock item | [optional]
**customer** | Option<**i32**> | Customer | [optional]
**delete_on_deplete** | Option<**bool**> | Delete this Stock Item when stock is depleted | [optional]
**expiry_date** | Option<[**String**](String.md)> | Expiry date for stock item. Stock will be considered expired after this date | [optional]
**in_stock** | **bool** |  | [readonly]
**is_building** | Option<**bool**> |  | [optional]
**link** | Option<**String**> | Link to external URL | [optional]
**notes** | Option<**String**> | Markdown notes (optional) | [optional]
**owner** | Option<**i32**> | Select Owner | [optional]
**packaging** | Option<**String**> | Packaging this stock item is stored in | [optional]
**parent** | **i32** | Parent stock item | [readonly]
**purchase_order** | Option<**i32**> | Purchase order for this stock item | [optional]
**purchase_order_reference** | Option<**String**> |  | [optional][readonly]
**sales_order** | Option<**i32**> |  | [optional]
**sales_order_reference** | Option<**String**> |  | [optional][readonly]
**status** | Option<[**models::StockItemStatusEnum**](StockItemStatusEnum.md)> |  | [optional]
**status_text** | **String** |  | [readonly]
**status_custom_key** | Option<**i32**> | Additional status information for this item | [optional]
**supplier_part** | Option<**i32**> | Select a matching supplier part for this stock item | [optional]
**sku** | Option<**String**> |  | [optional][readonly]
**mpn** | Option<**String**> |  | [optional][readonly]
**barcode_hash** | **String** | Unique hash of barcode data | [readonly]
**updated** | Option<**String**> | Timestamp of last update | [optional][readonly]
**stocktake_date** | Option<[**String**](String.md)> |  | [optional][readonly]
**purchase_price** | Option<**String**> | Purchase price of this stock item, per unit or pack | [optional]
**purchase_price_currency** | Option<**String**> | Purchase currency of this stock item | [optional]
**use_pack_size** | Option<**bool**> | Use pack size when adding: the quantity defined is the number of packs | [optional]
**serial_numbers** | Option<**String**> | Enter serial numbers for new items | [optional]
**allocated** | Option<**f64**> |  | [optional][readonly]
**expired** | Option<**bool**> |  | [optional][readonly]
**installed_items** | Option<**i32**> |  | [optional][readonly]
**child_items** | Option<**i32**> |  | [optional][readonly]
**stale** | Option<**bool**> |  | [optional][readonly]
**tracking_items** | Option<**i32**> |  | [optional][readonly]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


