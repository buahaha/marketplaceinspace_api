pub mod links;
pub mod token;
pub mod account;
pub mod stream;

// Rate Limiting
// REST API
// 120 requests per second. Excess requests will receive 
// HTTP 429 error. This restriction is applied against 
// the requesting IP address.

// Streaming API
// 20 active streams. Requests above this threshold will 
// be rejected. This restriction is applied against the 
// requesting IP address.

// Connection Limiting
// Client is allowed to make no more than 2 new connections 
// per second. Excess connections will be rejected. 
// For more details on making persistent connections, 
// please refer to the Best Practices page.




// BEST PRACTICES
// Introduction
// The OANDA API development team strives to bring the best 
// overall experience for our API users. Here is a set of 
// best practices to use the API as efficiently as possible.

// Connection Limits
// To provide equal resources to all clients, we recommend 
// limiting both the number of new connections per second, 
// and the number of requests per second made on a persistent 
// connection (see above/below).

// For new connections, we recommend you limit this to twice 
// per second (2/s). Establishing a new TCP and SSL connection 
// is expensive for both client and server. To allow a better 
// experience, using a persistent connection will allow more 
// requests to be performed on an established connection.

// For an established connection, we recommend limiting this 
// to one hundred per second (100/s). Please see following 
// section on persistent connections to learn how to maintain 
// a connection once it is established.

// HTTP Persistent Connection
// This keeps the underlying TCP connection active for multiple 
// requests/responses. Subsequent requests will result in reduced 
//latency as the TCP handshaking process is no longer required.

// If you are using an HTTP 1.0 client, ensure it supports the 
// keep-alive directive and submit the header Connection: Keep-Alive 
// with your request.

// Keep-Alive is part of the protocol in HTTP 1.1 and enabled by 
// default on compliant clients. However, you will have to ensure your 
// implementation does not set the connection header to other values.

// Retrieving and updating account state
// The OANDA v20 API is designed to make it simple and easy to obtain 
// the entire state of your account with an initial Account Details request, 
// and then keep that snapshot completely up to date with repeated 
// Poll Account Updates requests. This allows an application to keep a full 
// and consistent view of an account with a simple update loop.

// Starting up

// When your application first connects it should issue the Account Details 
// request and store the returned Account snapshot, which represents a complete 
// and self consistent view of everything in your account including all pending 
// orders, trades, and positions. The application should also store the returned 
// TransactionID object for use in future Poll Account Update calls.

// Updating

// In order to keep the Account snapshot up to date the application should send 
// the Poll Account Updates request, providing the last TransactionID returned 
// in the initial Account Details request or the most recent Poll Account Updates 
// request. The response contains anything that’s changed in the account since 
// the supplied TransactionID and comes in two types, AccountChanges and AccountState.

// The AccountChanges object represents any changes to Orders, Trades, Positions, 
// or balance caused by Transactions processed against the account. To keep your 
// Account snapshot up to date simply add, remove, or replace the entities in the 
// Account snapshot with those specified in the AccountChanges object. This type 
// of change is likely to be infrequent as Transactions only occur when the user 
// makes a request or an order triggers.

// The AccountState object represents an Account’s current price-dependent state. 
// This is any value in the Account snapshot that changes when prices change, 
// such as unrealized PL, Net Asset Value, or Trailing Stop Loss state. 
// To keep your Account snapshot up to date simply replace the entities 
// in the Account snapshot with those specified in the AccountState object. 
// This type of change is likely to be frequent as prices can change more 
// than once a second.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
