#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;
extern crate validator;
#[macro_use]
extern crate validator_derive;

//use hdk::prelude::*;
use hdk_proc_macros::zome;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult
};

use hdk::holochain_core_types::{
    entry::Entry,
    //dna::entry_types::Sharing,
    link::LinkMatch
};

use hdk::holochain_json_api::{
    json::JsonString,
    json::RawString,
    error::JsonError
};

use hdk::utils::get_links_and_load_type;

pub static MESSAGE_ENTRY: &str = "message";
pub static MESSAGE_LINK_TYPE_TO: &str = "message_in";

use hdk::holochain_persistence_api::cas::content::{
    AddressableContent,
    Address
};

pub mod message;
pub mod member;
pub mod anchor;

// see https://developer.holochain.org/api/0.0.46-alpha1/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Post {
    message: String,
    timestamp: u64,
    author_id: Address,
}

#[zome]
mod hello_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    /*
    #[zome_fn("hc_public")]
    pub fn hello_holo() -> ZomeApiResult<String> {
        Ok("Hello Holo".into())
    }

    #[entry_def]
    fn post_entry_def() -> ValidatingEntryType {
        entry!(
            name: "post",
            description: "blog post",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | validation_data: hdk::EntryValidationData<Post>| {
                
                match validation_data {
                    hdk::EntryValidationData::Create{ entry, .. } => {
                        const MAX_LENGTH: usize = 140;
                        if entry.message.len() <= MAX_LENGTH {
                           Ok(())
                        } else {
                           Err("Post too long".into())
                        }
                    },
                    _ => Ok(()),
                }
            },
            links: [
                from!(
                    "%agent_id",
                    link_type: "author_post",
                    validation_package: || {
                       hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_validation_data: hdk::LinkValidationData| {
                       Ok(())
                    }
                )
            ]
        )
    }*/

    #[entry_def]
    pub fn message_entry_def() -> ValidatingEntryType {
        message::message_definition()
    }

    #[entry_def]
    pub fn member_entry_def() -> ValidatingEntryType {
        member::member_definition()
    }

    #[entry_def]
    pub fn anchor_entry_def() -> ValidatingEntryType {
        anchor::anchor_definition()
    }

    /*
    #[zome_fn("hc_public")]
    pub fn create_person(person: Person) -> ZomeApiResult<Address> {
        let entry = Entry::App("person".into(), person.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    pub fn retrieve_person(address: Address) -> ZomeApiResult<Person> {
        hdk::utils::get_as_type(address)
    }
    

    #[zome_fn("hc_public")]
    pub fn create_post(message: String, timestamp: u64) -> ZomeApiResult<Address> {

        let post = Post {
            message: message,
            timestamp: timestamp,
            author_id: hdk::AGENT_ADDRESS.clone(),
        };

        let agent_addr = hdk::AGENT_ADDRESS.clone().into();
        let entry = Entry::App("post".into(), post.into());
        let post_addr = hdk::commit_entry(&entry)?;

        hdk::link_entries(&agent_addr, &post_addr, "author_post", "")?;

        Ok(post_addr)
    }

    #[zome_fn("hc_public")]
    pub fn retrieve_posts(agent_addr: Address) -> ZomeApiResult<Vec<Post>> {

        hdk::utils::get_links_and_load_type(&agent_addr, LinkMatch::Exactly("author_post"), LinkMatch::Any)
    }*/

    #[zome_fn("hc_public")]
    pub fn get_agent_id() -> ZomeApiResult<Address> {

        Ok(hdk::AGENT_ADDRESS.clone())
    }

    /*
    #[zome_fn("hc_public")]
    pub fn create_and_anchor_conv() -> ZomeApiResult<Address> {

        conversation.

    }*/

    #[zome_fn("hc_public")]
	pub fn get_all_participants() -> ZomeApiResult<Vec<member::Member>> {

        let anchor_entry = Entry::App(
            "anchor".into(),
            RawString::from("members").into(),
        );
        let anchor_address = hdk::entry_address(&anchor_entry)?;
        let result = get_links_and_load_type(&anchor_address, LinkMatch::Exactly("member"), LinkMatch::Any)?;
        Ok(result)
	}

    #[zome_fn("hc_public")]
    pub fn get_member_info(address: Address) -> ZomeApiResult<Vec<member::Member>> {
        get_links_and_load_type(&address, LinkMatch::Exactly("info"), LinkMatch::Any)
    }

    #[zome_fn("hc_public")]
    pub fn get_self_info() -> ZomeApiResult<Vec<member::Member>> {
        let addr = hdk::AGENT_ADDRESS.clone().into();
        get_links_and_load_type(&addr, LinkMatch::Exactly("info"), LinkMatch::Any)
    }

    #[zome_fn("hc_public")]
    pub fn get_all_messages() -> ZomeApiResult<Vec<message::Message>> {

        let anchor_entry = Entry::App(
        "anchor".into(),
        RawString::from("messages").into(),
        );
        let anchor_address = hdk::entry_address(&anchor_entry)?;
        let result = get_links_and_load_type(&anchor_address, LinkMatch::Exactly("message"), LinkMatch::Any)?;
        Ok(result)        
    }

    #[zome_fn("hc_public")]
    pub fn post_message(message_spec: message::MessageSpec) -> ZomeApiResult<Address> {

        //get messages anchor address, create anchor if it doesnt exist
        let anchor_entry = Entry::App(
            "anchor".into(),
            RawString::from("messages").into(),
        );
        let anchor_address = anchor_entry.address();

        if hdk::get_entry(&anchor_address)?.is_none() {
            hdk::commit_entry(&anchor_entry)?;
        }

        //create and post message
        let message = message::Message::from_spec(
        &message_spec,
        &hdk::AGENT_ADDRESS.to_string());

        let message_entry = Entry::App(
            "message".into(),
            message.into(),
        );

        let message_addr = hdk::commit_entry(&message_entry)?;

        //link message to anchror
        hdk::link_entries(&anchor_address, &message_addr, "message", "")?;

        Ok(message_addr)

    }

    #[zome_fn("hc_public")]
    pub fn join_conversation(name: String) -> ZomeApiResult<Address> {

        //get members anchor address, create members anchor if it doesnt already exist
        let anchor_entry = Entry::App(
            "anchor".into(),
            RawString::from("members").into(),
        );
        let anchor_address = anchor_entry.address();

        if hdk::get_entry(&anchor_address)?.is_none() {
            hdk::commit_entry(&anchor_entry)?;
        }

        //add another entry for yourself if it doesn't already exist
        let member_entry = Entry::App(
            "member".into(),
            member::Member {
                name,
                address: hdk::AGENT_ADDRESS.clone().into(),
            }.into()
        );
        let member_addr = member_entry.address();
        if hdk::get_entry(&member_addr)?.is_none() {
            hdk::commit_entry(&member_entry)?;
        }

        //link agent address to this entry
        let agent_addr = hdk::AGENT_ADDRESS.clone().into();
        hdk::link_entries(&agent_addr, &member_addr, "info", "")?;

        //link entry to members anchor
        hdk::link_entries(&anchor_address, &member_addr, "member", "")?;

        Ok(member_addr)
    }
}
