pub mod abi;
pub mod settings;

use alloy_primitives::Address;
use anyhow::Result;
use async_openai::{
    types::{
        CreateAssistantRequest, CreateMessageRequest, CreateRunRequest, CreateThreadRequestArgs,
        MessageContent, RunStatus,
    },
    Client,
};
use settings::Settings;
use std::fs;

use crate::abi::get_abi;

pub async fn run(address: Address, prompt: String, settings: Settings) -> Result<()> {
    let client = Client::new();
    let query = [("limit", "10")];

    let assistants = client.assistants().list(&query).await?;

    let assistant_object = assistants
        .data
        .into_iter()
        .find(|assistant| assistant.name == Some("castGPT-11".to_string()));

    let assistant_object = match assistant_object {
        Some(assistant) => assistant,
        None => {
            let assistant_request = CreateAssistantRequest {
                name: Some("castGPT-11".to_string()),
                description: Some(
                    "castGPT generates forge cast commands based on the abi provided".to_string(),
                ),
                model: "gpt-4".to_string(),
                instructions: Some(fs::read_to_string("seed-prompt.txt").unwrap()),
                tools: None,
                file_ids: None,
                metadata: None,
            };
            client.assistants().create(assistant_request).await.unwrap()
        }
    };

    let create_thread_request = CreateThreadRequestArgs::default().build()?;
    let thread_object = client.threads().create(create_thread_request).await?;

    let abi = get_abi(address.to_string(), ethers_core::types::Chain::Mainnet)
        .await
        .unwrap();

    let create_message_request = CreateMessageRequest {
        role: "user".to_string(),
        content: format!(
            "The contract address is: {}. Rpc url is: {}. Abi is: {}. My task is {}",
            address, settings.rpc_url, abi, prompt
        ),
        file_ids: None,
        metadata: None,
    };

    println!("create message request: {:#?}", create_message_request);

    let _message_object = client
        .threads()
        .messages(&thread_object.id)
        .create(create_message_request)
        .await?;

    let create_run_request = CreateRunRequest {
      assistant_id: assistant_object.id,
      model: Some("gpt-4".to_string()),
      instructions: Some("Generate the cast commands. Do not provide any other text or special characters besides the commands. Do not include an abi option flag. Only use the function signature format specified in the examples provided. the rpc-url option should always be --rpc-url. the cast command should always have 'call' directly after cast".to_string()),
      tools: None,
      metadata: None,
  };

    let run_object = client
        .threads()
        .runs(&thread_object.id)
        .create(create_run_request)
        .await;

    let run_status = run_object.unwrap().clone();

    while run_status.status != RunStatus::Completed {
        let run_object = client
            .threads()
            .runs(&thread_object.id)
            .retrieve(&run_status.id)
            .await;
        let status = &run_object.unwrap().clone();
        println!("run object: {status:#?}");
        if status.status == RunStatus::Completed {
            println!("completed run object: {status:#?}");
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    let message = client
        .threads()
        .messages(&run_status.thread_id)
        .list(&query)
        .await?;

    let assistant_res = message.data[0].content.clone();
    let assistant_res = assistant_res[0].clone();

    let cast_cmd = match assistant_res {
        MessageContent::Text(text) => Ok(text.text.value.trim().to_string().clone()),
        _ => {
            println!("wrong response type");
            Err("".to_string())
        }
    };

    let cast_cmd = cast_cmd.unwrap();
    let cast_cmd = cast_cmd.split("\n").collect::<Vec<&str>>();

    println!("cast cmd: {:#?}", cast_cmd);

    //format the string of cast commands

    Ok(())
}
