use metashrew_support::index_pointer::KeyValuePointer;
use metashrew_support::compat::to_arraybuffer_layout;

use alkanes_runtime::{
  declare_alkane, message::MessageDispatch, storage::StoragePointer, token::Token,
  runtime::AlkaneResponder
};

use alkanes_support::{
  cellpack::Cellpack, id::AlkaneId,
  parcel::{AlkaneTransfer, AlkaneTransferParcel}, response::CallResponse
};

use anyhow::Result;
use std::sync::Arc;

#[derive(Default)]
pub struct ArbuzOrbitalInstance(());

impl AlkaneResponder for ArbuzOrbitalInstance {}

#[derive(MessageDispatch)]
enum ArbuzOrbitalInstanceMessage {
  #[opcode(0)]
  Initialize {
    index: u128,
  },

  #[opcode(99)]
  #[returns(String)]
  GetName,

  #[opcode(100)]
  #[returns(String)]
  GetSymbol,

  #[opcode(101)]
  #[returns(u128)]
  GetTotalSupply,

  #[opcode(998)]
  #[returns(String)]
  GetCollectionIdentifier,

  #[opcode(999)]
  #[returns(Vec<u8>)]
  GetCollectionAlkaneId,

  #[opcode(1000)]
  #[returns(Vec<u8>)]
  GetData,

  #[opcode(1001)]
  #[returns(String)]
  GetContentType,

  #[opcode(1002)]
  #[returns(String)]
  GetAttributes,
}

impl Token for ArbuzOrbitalInstance {
  fn name(&self) -> String {
    let name = String::from("Magic Arbuz");

    format!("{} #{}", name, self.index())
  }

  fn symbol(&self) -> String {
    let symbol = String::from("magic-arbuz");

    format!("{}-{}", symbol, self.index())
  }
}

impl ArbuzOrbitalInstance {
  fn initialize(&self, index: u128) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    self.observe_initialization()?;
    
    self.set_collection_alkane_id(&context.caller);
    self.set_index(index);

    response.alkanes.0.push(AlkaneTransfer {
      id: context.myself.clone(),
      value: 1u128,
    });

    Ok(response)
  }

  fn get_name(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.name().into_bytes().to_vec();

    Ok(response)
  }

  fn get_symbol(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = self.symbol().into_bytes().to_vec();

    Ok(response)
  }

  fn get_total_supply(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    response.data = (&1u128.to_le_bytes()).to_vec();

    Ok(response)
  }

  fn get_data(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let collection_id = self.collection_ref();
    
    let cellpack = Cellpack {
      target: collection_id,
      inputs: vec![1000, self.index()],
    };
    
    let call_response = self.staticcall(
      &cellpack,
      &AlkaneTransferParcel::default(),
      self.fuel()
    )?;
    
    response.data = call_response.data;

    Ok(response)
  }

  fn get_content_type(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);
    response.data = String::from("text/javascript").into_bytes().to_vec();
    Ok(response)
  }

  fn get_attributes(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let collection_id = self.collection_ref();
    
    let cellpack = Cellpack {
      target: collection_id,
      inputs: vec![999, self.index()],
    };
    
    let call_response = self.staticcall(
      &cellpack,
      &AlkaneTransferParcel::default(),
      self.fuel()
    )?;
    
    response.data = call_response.data;

    Ok(response)
  }

  fn set_collection_alkane_id(&self, id: &AlkaneId) {
    let mut bytes = Vec::with_capacity(32);
    bytes.extend_from_slice(&id.block.to_le_bytes());
    bytes.extend_from_slice(&id.tx.to_le_bytes());
    
    self.collection_alkane_id_pointer().set(Arc::new(bytes));
  }

  fn collection_alkane_id_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/collection-alkane-id")
  }

  fn get_collection_identifier(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let collection = self.collection_ref();
    response.data = format!("{}:{}", collection.block, collection.tx).into_bytes();

    Ok(response)
  }

  fn get_collection_alkane_id(&self) -> Result<CallResponse> {
    let context = self.context()?;
    let mut response = CallResponse::forward(&context.incoming_alkanes);

    let collection = self.collection_ref();
    let mut bytes = Vec::with_capacity(32);
    bytes.extend_from_slice(&collection.block.to_le_bytes());
    bytes.extend_from_slice(&collection.tx.to_le_bytes());
    
    response.data = bytes;

    Ok(response)
  }

  fn collection_ref(&self) -> AlkaneId {
    let data = self.collection_alkane_id_pointer().get();
    if data.len() == 0 {
       panic!("Collection reference not found");
    }
    
    let bytes = data.as_ref();
    AlkaneId {
      block: u128::from_le_bytes(bytes[0..16].try_into().unwrap()),
      tx: u128::from_le_bytes(bytes[16..32].try_into().unwrap()),
    }
  }

  fn index_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/index")
  }

  fn index(&self) -> u128 {
    self.index_pointer().get_value::<u128>()
  }

  fn set_index(&self, index: u128) {
    self.index_pointer().set_value::<u128>(index);
  }
}

declare_alkane! {
  impl AlkaneResponder for ArbuzOrbitalInstance {
    type Message = ArbuzOrbitalInstanceMessage;
  }
}
