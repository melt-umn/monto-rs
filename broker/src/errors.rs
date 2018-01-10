use notify::Error as NotifyError;

use service::{ServiceConnectError, ServiceConnectErrorKind};

error_chain! {
    foreign_links {
        Notify(NotifyError)
            #[doc = "An error setting up the notifier."];
    }
    links {
        ServiceConnect(ServiceConnectError, ServiceConnectErrorKind)
            #[doc = "An error connecting to a service."];
    }
}
