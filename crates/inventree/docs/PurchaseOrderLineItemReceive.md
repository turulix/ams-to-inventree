# PurchaseOrderLineItemReceive

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**line_item** | **i32** |  | 
**location** | Option<**i32**> | Select destination location for received items | [optional]
**quantity** | **String** |  | 
**batch_code** | Option<**String**> | Enter batch code for incoming stock items | [optional][default to ]
**expiry_date** | Option<[**String**](String.md)> | Enter expiry date for incoming stock items | [optional]
**serial_numbers** | Option<**String**> | Enter serial numbers for incoming stock items | [optional][default to ]
**status** | Option<**i32**> | Stock item status code  * `10` - OK * `50` - Attention needed * `55` - Damaged * `60` - Destroyed * `65` - Rejected * `70` - Lost * `75` - Quarantined * `85` - Returned  Additional custom status keys may be retrieved from the 'stock_status_retrieve' call. | [optional][default to 10]
**packaging** | Option<**String**> | Override packaging information for incoming stock items | [optional][default to ]
**note** | Option<**String**> | Additional note for incoming stock items | [optional][default to ]
**barcode** | Option<**String**> | Scanned barcode | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


