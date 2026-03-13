# BuildComplete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accept_overallocated** | Option<[**models::AcceptOverallocatedEnum**](AcceptOverallocatedEnum.md)> | How do you want to handle extra stock items assigned to the build order  * `reject` - Not permitted * `accept` - Accept as consumed by this build order * `trim` - Deallocate before completing this build order | [optional][default to Reject]
**accept_unallocated** | Option<**bool**> | Accept that stock items have not been fully allocated to this build order | [optional][default to false]
**accept_incomplete** | Option<**bool**> | Accept that the required number of build outputs have not been completed | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


