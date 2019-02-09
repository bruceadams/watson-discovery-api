# Expansions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expansions** | [**Vec<::models::Expansion>**](Expansion.md) | An array of query expansion definitions.    Each object in the **expansions** array represents a term or set of terms that will be expanded into other terms. Each expansion object can be configured as bidirectional or unidirectional. Bidirectional means that all terms are expanded to all other terms in the object. Unidirectional means that a set list of terms can be expanded into a second list of terms.   To create a bi-directional expansion specify an **expanded_terms** array. When found in a query, all items in the **expanded_terms** array are then expanded to the other items in the same array.   To create a uni-directional expansion, specify both an array of **input_terms** and an array of **expanded_terms**. When items in the **input_terms** array are present in a query, they are expanded using the items listed in the **expanded_terms** array.   | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


