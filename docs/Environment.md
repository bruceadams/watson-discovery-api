# Environment

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment_id** | **String** | Unique identifier for the environment. | [optional] 
**name** | **String** | Name that identifies the environment. | [optional] [default to ]
**description** | **String** | Description of the environment. | [optional] [default to ]
**created** | **String** | Creation date of the environment, in the format `yyyy-MM-dd'T'HH:mm:ss.SSS'Z'` | [optional] 
**updated** | **String** | Date of most recent environment update, in the format `yyyy-MM-dd'T'HH:mm:ss.SSS'Z'` | [optional] 
**status** | **String** | Current status of the environment. `resizing` is displayed when a request to increase the environment size has been made, but is still in the process of being completed. | [optional] 
**read_only** | **bool** | If `true`, the environment contains read-only collections that are maintained by IBM. | [optional] 
**size** | **String** | Current size of the environment. | [optional] 
**requested_size** | **String** | The new size requested for this environment. Only returned when the environment *status* is `resizing`.  *Note:* Querying and indexing can still be performed during an environment upsize. | [optional] 
**index_capacity** | [***::models::IndexCapacity**](IndexCapacity.md) |  | [optional] 
**search_status** | [***::models::SearchStatus**](SearchStatus.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


