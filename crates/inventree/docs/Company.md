# Company

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** | Company name | 
**description** | Option<**String**> | Description of the company | [optional]
**website** | Option<**String**> | Company website URL | [optional]
**phone** | Option<**String**> | Contact phone number | [optional]
**email** | Option<**String**> |  | [optional][default to ]
**currency** | **String** | Default currency used for this supplier | 
**contact** | Option<**String**> | Point of contact | [optional]
**link** | Option<**String**> | Link to external company information | [optional]
**image** | Option<**String**> |  | [optional]
**active** | Option<**bool**> | Is this company active? | [optional]
**is_customer** | Option<**bool**> | Do you sell items to this company? | [optional]
**is_manufacturer** | Option<**bool**> | Does this company manufacture parts? | [optional]
**is_supplier** | Option<**bool**> | Do you purchase items from this company? | [optional]
**parts_supplied** | **i32** |  | [readonly]
**parts_manufactured** | **i32** |  | [readonly]
**remote_image** | Option<**String**> | URL of remote image file | [optional]
**tax_id** | Option<**String**> | Company Tax ID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


