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

//use hdk::prelude::*;
use hdk_proc_macros::zome;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult
};

use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::cas::content::Address;


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
    */

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
    }

    #[zome_fn("hc_public")]
    pub fn get_agent_id() -> ZomeApiResult<Address> {

        Ok(hdk::AGENT_ADDRESS.clone())
    }


}