# LogQueryResponseResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment_id** | **String** | The environment ID that is associated with this log entry. | [optional] 
**customer_id** | **String** | The **customer_id** label that was specified in the header of the query or event API call that corresponds to this log entry. | [optional] 
**document_type** | **String** | The type of log entry returned.   **query** indicates that the log represents the results of a call to the single collection **query** method.   **event** indicates that the log represents  a call to the **events** API. | [optional] 
**natural_language_query** | **String** | The value of the **natural_language_query** query parameter that was used to create these results. Only returned with logs of type **query**.  **Note:** Other query parameters (such as **filter** or **deduplicate**) might  have been used with this query, but are not recorded. | [optional] 
**document_results** | [***::models::LogQueryResponseResultDocuments**](LogQueryResponseResultDocuments.md) |  | [optional] 
**created_timestamp** | **String** | Date that the log result was created. Returned in `YYYY-MM-DDThh:mm:ssZ` format. | [optional] 
**client_timestamp** | **String** | Date specified by the user when recording an event. Returned in `YYYY-MM-DDThh:mm:ssZ` format. Only returned with logs of type **event**. | [optional] 
**query_id** | **String** | Identifier that corresponds to the **natural_language_query** string used in the original or associated query. All **event** and **query** log entries that have the same original **natural_language_query** string also have them same **query_id**. This field can be used to recall all **event** and **query** log results that have the same original query (**event** logs do not contain the original **natural_language_query** field). | [optional] 
**session_token** | **String** | Unique identifier (within a 24-hour period) that identifies a single `query` log and any `event` logs that were created for it.  **Note:** If the exact same query is run at the exact same time on different days, the **session_token** for those queries might be identical. However, the **created_timestamp** differs.   **Note:** Session tokens are case sensitive. To avoid matching on session tokens that are identical except for case, use the exact match operator (`::`) when you query for a specific session token. | [optional] 
**collection_id** | **String** | The collection ID of the document associated with this event. Only returned with logs of type `event`. | [optional] 
**display_rank** | **i32** | The original display rank of the document associated with this event. Only returned with logs of type `event`. | [optional] 
**document_id** | **String** | The document ID of the document associated with this event. Only returned with logs of type `event`. | [optional] 
**event_type** | **String** | The type of event that this object respresents. Possible values are   -  `query` the log of a query to a collection   -  `click` the result of a call to the **events** endpoint. | [optional] 
**result_type** | **String** | The type of result that this **event** is associated with. Only returned with logs of type `event`. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


