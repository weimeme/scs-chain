
use sp_application_crypto::{AppCrypto, ByteArray};
use sp_core::H256;
use std::sync::Arc;
use sp_keystore::{Keystore, KeystorePtr};
use sp_consensus_babe::{AuthorityId, AuthorityPair, Randomness, Slot, VrfSignature, make_vrf_transcript, BabeApi};
use std::marker::PhantomData;
use sc_service::{
    error::Error as ServiceError, ChainSpec, Configuration, PartialComponents, TFullBackend,
    TFullClient, TaskManager,
};
use fc_rpc::pending::ConsensusDataProvider;

// /// Consensus data provider for Babe.
// pub struct AuraConsensusDataProvider<B, C> {
//     // slot duration
//     slot_duration: SlotDuration,
//     // phantom data for required generics
//     _phantom: PhantomData<(B, C)>,
// }