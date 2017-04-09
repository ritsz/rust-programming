/* super can be used to access parent modules */

use super::{datalink, use_server};

pub fn connect() {
    datalink();
    /* use_server access server. we access use_server through layer. parent of parent access */
    use_server();
}
