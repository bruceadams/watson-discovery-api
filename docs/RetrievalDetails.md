# RetrievalDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_retrieval_strategy** | **String** | Indentifies the document retrieval strategy used for this query. `relevancy_training` indicates that the results were returned using a relevancy trained model. `continuous_relevancy_training` indicates that the results were returned using the continuous relevancy training model created by result feedback analysis. `untrained` means the results were returned using the standard untrained model.   **Note**: In the event of trained collections being queried, but the trained model is not used to return results, the **document_retrieval_strategy** will be listed as `untrained`. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


