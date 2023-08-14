use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use starknet_types::felt::Felt;
use starknet_types::starknet_api::state::EntryPoint;
use starknet_types::starknet_api::state::EntryPointType;

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct SierraContractClass {
    pub sierra_program: Vec<Felt>,
    pub contract_class_version: String,
    pub entry_points_by_type: HashMap<EntryPointType, Vec<EntryPoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<String>,
}

#[cfg(test)]
mod tests {
    use starknet_types::felt::Felt;

    use starknet_types::abi_entry::{AbiEntry, AbiEntryType, FunctionAbiEntry};
    use starknet_types::contract_class::deprecated::rpc_contract_class::{
        ContractClassAbiEntryWithType, DeprecatedContractClass,
    };

    #[test]
    fn deserialize_contract_class_abi_entry_with_type() {
        let json_str = r#"{
            "inputs": [],
            "name": "getPublicKey",
            "outputs": [
                {
                    "name": "publicKey",
                    "type": "felt"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        }"#;

        let obj = serde_json::from_str::<ContractClassAbiEntryWithType>(json_str).unwrap();
        assert_eq!(obj.r#type, AbiEntryType::Function);
        assert_eq!(
            obj.entry,
            AbiEntry::Function(FunctionAbiEntry {
                name: "getPublicKey".to_string(),
                inputs: vec![],
                outputs: vec![
                    starknet_types::starknet_api::deprecated_contract_class::TypedParameter {
                        name: "publicKey".to_string(),
                        r#type: "felt".to_string(),
                    }
                ],
                state_mutability: Some("view".to_string()),
            })
        );

        let json_str = r#"{
            "inputs": [
                {
                    "name": "newPublicKey",
                    "type": "felt"
                }
            ],
            "name": "setPublicKey",
            "outputs": [],
            "type": "function"
        }"#;

        let obj = serde_json::from_str::<ContractClassAbiEntryWithType>(json_str).unwrap();
        assert_eq!(obj.r#type, AbiEntryType::Function);
        assert_eq!(
            obj.entry,
            AbiEntry::Function(FunctionAbiEntry {
                name: "setPublicKey".to_string(),
                inputs: vec![
                    starknet_types::starknet_api::deprecated_contract_class::TypedParameter {
                        name: "newPublicKey".to_string(),
                        r#type: "felt".to_string(),
                    }
                ],
                outputs: vec![],
                state_mutability: None,
            })
        );

        let json_str = r#"{
            "inputs": [
                {
                    "name": "publicKey",
                    "type": "felt"
                }
            ],
            "name": "constructor",
            "outputs": [],
            "type": "constructor"
        }"#;

        let obj = serde_json::from_str::<ContractClassAbiEntryWithType>(json_str).unwrap();
        assert_eq!(obj.r#type, AbiEntryType::Constructor);
        assert_eq!(
            obj.entry,
            AbiEntry::Function(FunctionAbiEntry {
                name: "constructor".to_string(),
                inputs: vec![
                    starknet_types::starknet_api::deprecated_contract_class::TypedParameter {
                        name: "publicKey".to_string(),
                        r#type: "felt".to_string(),
                    }
                ],
                outputs: vec![],
                state_mutability: None,
            })
        );
    }

    #[test]
    fn deserialize_deprecated_contract_class() {
        let json_str = r#"{
            "abi": [
                {
                    "inputs": [],
                    "name": "getPublicKey",
                    "outputs": [
                        {
                            "name": "publicKey",
                            "type": "felt"
                        }
                    ],
                    "stateMutability": "view",
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "name": "newPublicKey",
                            "type": "felt"
                        }
                    ],
                    "name": "setPublicKey",
                    "outputs": [],
                    "type": "function"
                },
                {
                    "inputs": [
                        {
                            "name": "publicKey",
                            "type": "felt"
                        }
                    ],
                    "name": "constructor",
                    "outputs": [],
                    "type": "constructor"
                }
            ],
            "program": "",
            "entry_points_by_type": {
                "CONSTRUCTOR": [],
                "L1_HANDLER": [],
                "EXTERNAL": [
                    {
                        "selector": "0xAAE3B5E8",
                        "offset": "0x1"
                    },
                    {
                        "selector": "0xAAE3B5E9",
                        "offset": "0x2"
                    }
                ]
            }
        }"#;

        let obj = serde_json::from_str::<DeprecatedContractClass>(json_str).unwrap();
        assert_eq!(obj.abi.len(), 3);
        assert_eq!(obj.entry_points_by_type.external.len(), 2);
    }

    #[test]
    fn deserialize_sierra_contract_class() {
        let json_str = r#"{
            "sierra_program": ["0xAA", "0xBB"],
            "contract_class_version": "1.0",
            "entry_points_by_type": {
                "EXTERNAL": [
                    {
                        "selector": "0xAAE3B5E8",
                        "function_idx": 1
                    },
                    {
                        "selector": "0xAAE3B5E9",
                        "function_idx": 2
                    }
                ]
            },
            "abi": "H4sIAAAAAAAA/8tIzcnJVyjPL8pJUQQAlQYXAAAA"
        }"#;
        let obj = serde_json::from_str::<super::SierraContractClass>(json_str).unwrap();
        assert_eq!(obj.sierra_program.len(), 2);
        assert_eq!(obj.contract_class_version, "1.0".to_string());
        assert_eq!(obj.entry_points_by_type.len(), 1);
        assert_eq!(
            obj.entry_points_by_type
                .get(&starknet_types::starknet_api::state::EntryPointType::External)
                .unwrap()
                .len(),
            2
        );
        assert_eq!(obj.abi.unwrap(), "H4sIAAAAAAAA/8tIzcnJVyjPL8pJUQQAlQYXAAAA".to_string());
        assert_eq!(
            obj.entry_points_by_type
                .get(&starknet_types::starknet_api::state::EntryPointType::External)
                .unwrap()[0]
                .selector
                .0,
            starknet_types::starknet_api::hash::StarkFelt::from(
                Felt::from_prefixed_hex_str("0xAAE3B5E8").unwrap()
            )
        );
    }
}
