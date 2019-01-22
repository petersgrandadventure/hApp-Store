#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate utils;
#[macro_use]
extern crate holochain_core_types_derive;

pub mod categories_fn;

use crate::categories_fn::App;
use hdk::{
    holochain_core_types::{
        cas::content::Address,
        dna::entry_types::Sharing,
        json::RawString,
    },
    error::ZomeApiResult
};

define_zome! {
    entries: [
        entry!(
            name: "category_anchor",
            description: "",
            sharing: Sharing::Public,
            native_type: RawString,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_name: RawString, _ctx: hdk::ValidationData| {
                Ok(())
            },

            links: [
                to!(
                    "app",
                    tag: "contains",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                from!(
                    "app",
                    tag: "in",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "tag_anchor",
            description: "",
            sharing: Sharing::Public,
            native_type: RawString,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |_name: RawString, _ctx: hdk::ValidationData| {
                Ok(())
            },

            links: [
                to!(
                    "app",
                    tag: "contains",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                from!(
                    "app",
                    tag: "in",

                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            add_app_to_category: {
                inputs:|app_address: Address, category: String|,
                outputs: | result: ZomeApiResult<()> |,
                handler: categories_fn::handle_add_app_to_category
            }
            add_app_to_tag: {
                inputs:|app_address: Address, tag: String|,
                outputs: | result: ZomeApiResult<()> |,
                handler: categories_fn::handle_add_app_to_tag
            }
            get_apps_by_category: {
                inputs:|category: String|,
                outputs: |result: ZomeApiResult<Vec<utils::GetLinksLoadElement<App>>>|,
                handler: categories_fn::handle_get_apps_by_category
            }
            get_apps_by_tag: {
                inputs:|tag: String|,
                outputs: |result: ZomeApiResult<Vec<utils::GetLinksLoadElement<App>>>|,
                handler: categories_fn::handle_get_apps_by_tag
            }
        }
    }
}
