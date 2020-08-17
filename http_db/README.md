# Rust API client for csmlenginems

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ConversationsApi* | [**close_all_conversations**](docs/ConversationsApi.md#close_all_conversations) | **post** /conversations/close | Close all conversations
*ConversationsApi* | [**close_conversation**](docs/ConversationsApi.md#close_conversation) | **post** /conversations/{conversationId}/close | Close a specific conversations
*ConversationsApi* | [**create_conversation**](docs/ConversationsApi.md#create_conversation) | **post** /conversations | Create a conversation
*ConversationsApi* | [**get_conversation**](docs/ConversationsApi.md#get_conversation) | **get** /conversations/{conversationId} | Retrieve a conversation
*ConversationsApi* | [**get_latest_open**](docs/ConversationsApi.md#get_latest_open) | **get** /conversations/latest | Get the latest open conversation
*ConversationsApi* | [**update_conversation**](docs/ConversationsApi.md#update_conversation) | **put** /conversations/{conversationId} | Update a conversation (flow, step, last_interaction_at)
*InteractionsApi* | [**get_interaction**](docs/InteractionsApi.md#get_interaction) | **get** /interactions/{interactionId} | Get an interaction
*InteractionsApi* | [**get_interaction_status**](docs/InteractionsApi.md#get_interaction_status) | **get** /interactions/{interactionId}/status | Find if a given interaction was successfully processed
*InteractionsApi* | [**get_lock_status**](docs/InteractionsApi.md#get_lock_status) | **get** /interactions/lock | Find if the current user is currently processing an interaction
*InteractionsApi* | [**init_interaction**](docs/InteractionsApi.md#init_interaction) | **post** /interactions | Initialize a new interaction
*InteractionsApi* | [**update_interaction**](docs/InteractionsApi.md#update_interaction) | **put** /interactions/{interactionId} | Update an interaction's status
*MemoriesApi* | [**add_memories_bulk**](docs/MemoriesApi.md#add_memories_bulk) | **post** /conversations/{conversationId}/memories/bulk | Add an array of memories to an existing conversation
*MemoriesApi* | [**add_memory**](docs/MemoriesApi.md#add_memory) | **post** /conversations/{conversationId}/memories | Add a memory to an existing conversation
*MemoriesApi* | [**get_current_memories**](docs/MemoriesApi.md#get_current_memories) | **get** /conversations/{conversationId}/memories/current | Get all the current memories
*MemoriesApi* | [**get_past_memories**](docs/MemoriesApi.md#get_past_memories) | **get** /conversations/{conversationId}/memories/past | Get all the past memories (not in the current conversation)
*MessagesApi* | [**add_message**](docs/MessagesApi.md#add_message) | **post** /conversations/{conversationId}/messages | Add a message to an existing conversation
*MessagesApi* | [**add_messages_bulk**](docs/MessagesApi.md#add_messages_bulk) | **post** /conversations/{conversationId}/messages/bulk | Add an array of messages to a conversation
*NodesApi* | [**create_node**](docs/NodesApi.md#create_node) | **post** /conversations/{conversationId}/nodes | Add a new node to a conversation
*NodesApi* | [**get_conversation_nodes**](docs/NodesApi.md#get_conversation_nodes) | **get** /conversations/{conversationId}/nodes | Retrieve a conversation's nodes
*StateApi* | [**clear_full_state**](docs/StateApi.md#clear_full_state) | **delete** /state | Clear a client's full memory state
*StateApi* | [**delete_item_state**](docs/StateApi.md#delete_item_state) | **delete** /state/{compositeKey} | Delete a state item
*StateApi* | [**get_full_state**](docs/StateApi.md#get_full_state) | **get** /state | Get the full state for a client
*StateApi* | [**get_item_state**](docs/StateApi.md#get_item_state) | **get** /state/{compositeKey} | Get the state of the requested memory item. If it does not exist yet, a success code of 204 is returned
*StateApi* | [**set_item_state**](docs/StateApi.md#set_item_state) | **put** /state/{compositeKey} | Set a memory item to a given state


## Documentation For Models

 - [ClientModel](docs/ClientModel.md)
 - [ConversationModel](docs/ConversationModel.md)
 - [CreateConversationBody](docs/CreateConversationBody.md)
 - [CreateInteractionBody](docs/CreateInteractionBody.md)
 - [CreateMemoryBody](docs/CreateMemoryBody.md)
 - [CreateMessageBody](docs/CreateMessageBody.md)
 - [CreateNodeBody](docs/CreateNodeBody.md)
 - [Error](docs/Error.md)
 - [InlineObject](docs/InlineObject.md)
 - [InlineObject1](docs/InlineObject1.md)
 - [InlineObject2](docs/InlineObject2.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponse2001](docs/InlineResponse2001.md)
 - [InlineResponse2002](docs/InlineResponse2002.md)
 - [InteractionModel](docs/InteractionModel.md)
 - [MemoryModel](docs/MemoryModel.md)
 - [NodeModel](docs/NodeModel.md)
 - [SetStateBody](docs/SetStateBody.md)
 - [StateModel](docs/StateModel.md)
 - [UpdateConversationBody](docs/UpdateConversationBody.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


