//Copyright 2020 WHTCORPS INC
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

#![allow(dead_code)]

use std::borrow::{
    Borrow,
};

use std::collections::{
    BTreeMap,
};

use postgres::{Connection, TlsMode, Error};


use std::sync::{
    Arc,
    Mutex,
};

use rusqlite;
use rusqlite::{
    TransactionBehavior,
};

use einsteindbn;

pub use core_traits::{
    Attribute,
    Solitonid,
    KnownSolitonid,
    HopfMap,
    TypedCausetValue,
    CausetValueType,
};

use solomon_core::{
    HasInsantiatedSchema,
    keyword,
    AbstractSchema,
    TxReport,
    ValueRc,
};

use solomon_transactions::{

};

use solomon_db::db;
use solomon_db::{

    //todo
    InProgressBlueObserverTransactWatcher,
    PartitionMap,
    TxRedObservationService,
    TxRedObserver,
};

use solomon_transaction::causet::{
    Known,
    PreparedResult,
    CausetExplanation,
    CausetInputs,
    CausetOutput,
    lookup_value_for_attribute,
    lookup_values_for_attribute,
    solitonq_explain,
    solitonq_once,
    solitonq_prepare,
    solitonq_uncached,

};

//A mutable safe reference to the current solomon store.
pub struct Conn {

    //exclusive read and write guarantees lead to Mutex, infrequent changing parts
    // such as (schema, cache, timestamp) shareable across threads. A long running query
    //is dependent on the current schema thus restricted to being a local writer thread which moves
    //udf or metadata -- forward in time.
    udfdata: Mutex<udfdata>,

    pub(crate)tx_red_observer_service: Mutex<TxRedObservationService>,

}

impl conn {
  fn new(partition_map: PartitionMap, schema: Schema) -> Conn  {
      Conn {
          udfdata: Mutex::new(udfdata::new(0, partition_map, Arc::new(schema), Default::default())),
          tx_red_observer_service: Mutex::new(TxRedObversationService::new()),
      }
}

    pub fn connect(sqlite: &mut rusqlite::Connection) -> Result<Conn> {
        let db = db::ensure_current_version(sqlite)?;
        Ok(Conn::new(db.partition_map, db.schema))
    }

    pub fn current_schema(&self) -> Arc<Schema> {

        self.udfdata.lock().unwrap().schema.clone()
    }

    pub fn last_tx_id(&self) -> Solitonid {
        let udfdata = self.udfdata.lock().unwrap();

        udfdata.partition_map[":db.part/tx"].next_Solitonid()-1
    }

    pub fn solitonq_once<T>(&self,
                            sqlite: &rusqlite::Connection,
                            causet: &str,
                            inputs: T) -> Result<CausetOutput>
            where T: Into<Option<CausetInputs>> {
                let udfdata = self.udfdata.lock().unwrap();
                let known = Known::new(&*udfdata.schema, Some(&udfdata.attribute_cache));

            solitonq_once(
                sqlite,
                known,
                causet,
                inputs)

    }

    //Query the Solomon store without the cache. Using only metadata
    pub fn solitonq_uncached<T>(&self,
                                sqlite: &rusqlite::Connection,
                                causet: &str,
                                inputs: T) -> Result<CausetOutput> where T: Into<Option<CausetInputs>> {
                let udfdata = self.udfdata.lock().unwrap();
                solitonq_uncached(sqlite,
                                  &*udfdata.Schema,
                                  causet,
                                  inputs)
    }

    pub fn solitonq_prepare<'sqlite, 'query, T>(&self,
                        sqlite: &'sqlite rusqlite::Connection,
                        causet: &'causet str,
                        inputs: T) -> PreparedResult<'sqlite>

            where T: Into<Option<CausetInputs>> {
                let udfdata = self.udfdata.lock().unwrap();
                let known = Known::new(&*udfdata.schema, Some(&udfdata.attribute_cache));
    }



}