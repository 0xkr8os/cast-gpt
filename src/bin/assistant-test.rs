use async_openai::{
    types::{
        CreateAssistantRequest, CreateMessageRequest, CreateRunRequestArgs, CreateThreadRequestArgs,
    },
    Client,
};
use dotenv::dotenv;
use std::error::Error;
// use tracing_subscriber::{
//     prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
// };

// cargo run --bin assistant-test
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::fmt::layer())
    //     .with(EnvFilter::from_default_env())
    //     .init();
    dotenv().ok();

    let client = Client::new();
    let query = [("limit", "10")];

    let assistants = client.assistants().list(&query).await?;
    println!("assistants: {assistants:#?}");

    // check if an assistant with the name "castGPT" exists
    for assistant in assistants.data {
        if assistant.name == Some("castGPT".to_string()) {
            panic!("assistant already exists");
        }
    }

    let assistant_request = CreateAssistantRequest {
        name: Some("castGPT".to_string()),
        description : Some("castGPT generates forge cast commands that retrieve EVM data upon a users plain english request".to_string()),
        model : "gpt-4".to_string(),
        instructions :Some("**Prompt:**\n\nYou are castGPT, a language model trained to generate forge \"cast\" commands based on user demand. You have access to ABI data for Ethereum smart contracts, which helps you understand the available public functions and variables. Your task is to generate a proper cast command based on the user's intent.\n\n**Task:**\n\nThe user wants to perform a cast call command on an Ethereum account without publishing a transaction. They want to specify the destination, the signature of the function to call, and the optional arguments. If the user doesn't provide all the necessary information, you should prompt them for additional details.\n\n**Query:**\n\nPlease generate a forge \"cast\" command to perform a call on an Ethereum account without publishing a transaction. The command should follow the format:\n\n`cast call [options] to sig [args...]`\n\nMake sure to include the following information:\n\n1. Explain that the destination (to) can be either an ENS name or an address.\n2. Inform that the signature (sig) can be one of the following:\n   - A function fragment, such as `someFunction(uint256,bytes32)`.\n   - A selector and encoded calldata, for example, `0xcdba2fd40000000000000000000000000000000000000000000000000000000000007a69`.\n   - Only the function name, in which case castGPT will attempt to fetch the function signature from Etherscan.\n3. Describe the available query options, such as the block height to query at.\n4. Mention the available RPC options, including the RPC endpoint.\n\nFinally, provide an example to call the `balanceOf(address)` function on the WETH contract:\n\n`cast call 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 \"balanceOf(address)(uint256)\" 0x...`\n\nEnsure that the prompt is in English.\nEnsure that the only response is the command itself, without any additional text.".to_string()),
        tools : None,
        file_ids : None,
        metadata : None,
    };

    let assistant_object = client.assistants().create(assistant_request).await?;

    println!("assistant object: {assistant_object:#?}",);

    let create_thread_request = CreateThreadRequestArgs::default().build()?;
    let thread_object = client.threads().create(create_thread_request).await?;

    println!("thread object: {thread_object:#?}");

    let create_message_request = CreateMessageRequest {
        role: "user".to_string(),
        content: "hello world".to_string(),
        file_ids: None,
        metadata: None,
    };
    let message_object = client
        .threads()
        .messages(&thread_object.id)
        .create(create_message_request)
        .await?;

    println!("message object: {message_object:#?}");

    let create_run_request = CreateRunRequestArgs::default().build()?;
    let run_object = client
        .threads()
        .runs(&thread_object.id)
        .create(create_run_request)
        .await;

    println!("run object: {run_object:#?}");

    // let thread_runs = client
    //     .threads()
    //     .runs(&thread_object.id)
    //     .list(&query)
    //     .await?;

    // println!("runs for thread id {}: {thread_runs:#?}", thread_object.id);

    let thread_messages = client
        .threads()
        .messages(&thread_object.id)
        .list(&query)
        .await?;

    println!("thread messages: {thread_messages:#?}");

    // let thread_message_files = client
    //     .threads()
    //     .messages(&thread_object.id)
    //     .files("message_id")
    //     .list(&query)
    //     .await;
    // println!("thread message file: {thread_message_files:#?}");
    // let thread_run_steps = client
    //     .threads()
    //     .runs(&thread_object.id)
    //     .steps("run_id")
    //     .list(&query)
    //     .await;
    // println!("thread run steps: {thread_run_steps:#?}");

    let deleted_thread_object = client.threads().delete(&thread_object.id).await?;
    println!("deleted thread object: {deleted_thread_object:#?}");

    // let assistant_files = client.assistants().files("assistant_id").list(&query).await;
    // println!("assistant files: {assistant_files:#?}");

    Ok(())
}
