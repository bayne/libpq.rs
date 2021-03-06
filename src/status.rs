#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    /** The server's response was not understood. */
    BadResponse,
    /** Successful completion of a command returning no data. */
    CommandOk,
    /**
     * Copy In/Out (to and from server) data transfer started. This feature is currently used only
     * for streaming replication, so this status should not occur in ordinary applications.
     */
    CopyBoth,
    /** Copy In (to server) data transfer started. */
    CopyIn,
    /** Copy Out (from server) data transfer started. */
    CopyOut,
    /** The string sent to the server was empty. */
    EmptyQuery,
    /** A fatal error occurred. */
    FatalError,
    /** A nonfatal error (a notice or warning) occurred. */
    NonFatalError,
    /**
     * The `libpq::Result` contains a single result tuple from the current command. This status
     * occurs only when single-row mode has been selected for the query
     */
    SingleTuble,
    /** Successful completion of a command returning data (such as a `SELECT` or `SHOW`). */
    TupplesOk,
}

#[doc(hidden)]
impl From<pq_sys::ExecStatusType> for Status {
    fn from(status: pq_sys::ExecStatusType) -> Self {
        match status {
            pq_sys::PGRES_BAD_RESPONSE => Self::BadResponse,
            pq_sys::PGRES_COMMAND_OK => Self::CommandOk,
            pq_sys::PGRES_COPY_BOTH => Self::CopyBoth,
            pq_sys::PGRES_COPY_IN => Self::CopyIn,
            pq_sys::PGRES_COPY_OUT => Self::CopyOut,
            pq_sys::PGRES_EMPTY_QUERY => Self::EmptyQuery,
            pq_sys::PGRES_FATAL_ERROR => Self::FatalError,
            pq_sys::PGRES_NONFATAL_ERROR => Self::NonFatalError,
            pq_sys::PGRES_SINGLE_TUPLE => Self::SingleTuble,
            pq_sys::PGRES_TUPLES_OK => Self::TupplesOk,
        }
    }
}

#[doc(hidden)]
impl Into<pq_sys::_bindgen_ty_4> for Status {
    fn into(self) -> pq_sys::_bindgen_ty_4 {
        (&self).into()
    }
}

#[doc(hidden)]
impl Into<pq_sys::_bindgen_ty_4> for &Status {
    fn into(self) -> pq_sys::_bindgen_ty_4 {
        match *self {
            Status::BadResponse => pq_sys::PGRES_BAD_RESPONSE,
            Status::CommandOk => pq_sys::PGRES_COMMAND_OK,
            Status::CopyBoth => pq_sys::PGRES_COPY_BOTH,
            Status::CopyIn => pq_sys::PGRES_COPY_IN,
            Status::CopyOut => pq_sys::PGRES_COPY_OUT,
            Status::EmptyQuery => pq_sys::PGRES_EMPTY_QUERY,
            Status::FatalError => pq_sys::PGRES_FATAL_ERROR,
            Status::NonFatalError => pq_sys::PGRES_NONFATAL_ERROR,
            Status::SingleTuble => pq_sys::PGRES_SINGLE_TUPLE,
            Status::TupplesOk => pq_sys::PGRES_TUPLES_OK,
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQresStatus(self.into()) })
    }
}
