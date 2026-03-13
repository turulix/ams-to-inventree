# BarcodePoReceive

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**barcode** | **String** | Scanned barcode data | 
**supplier** | Option<**i32**> | Supplier to receive items from | [optional]
**purchase_order** | Option<**i32**> | PurchaseOrder to receive items against | [optional]
**location** | Option<**i32**> | Location to receive items into | [optional]
**line_item** | Option<**i32**> | Purchase order line item to receive items against | [optional]
**auto_allocate** | Option<**bool**> | Automatically allocate stock items to the purchase order | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


