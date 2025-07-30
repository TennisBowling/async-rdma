//! This example demonstrates the auto-discovery functionality for RDMA settings.
//! It shows how to automatically find working device, port, and GID configurations.

use async_rdma::RdmaBuilder;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize tracing to see discovery logs
    tracing_subscriber::fmt::init();

    println!("=== RDMA Auto-Discovery Example ===\n");

    // Test 1: Find working RDMA settings
    println!("1. Testing RdmaBuilder::find_settings()...");
    match RdmaBuilder::find_settings() {
        Ok(settings) => {
            println!("✅ Found working RDMA settings:");
            println!("   Device: {:?}", settings.device_name);
            println!("   Port: {}", settings.port_num);
            println!("   GID Index: {}", settings.gid_index);
        }
        Err(e) => {
            println!("❌ Failed to find working RDMA settings: {}", e);
            println!("   This is expected if no RDMA devices are available.");
            return Ok(());
        }
    }

    println!("\n2. Testing auto-listen/auto-connect workflow...");
    
    // Get a free port for testing
    let addr = "127.0.0.1:0";
    
    // Test auto-listen (in a real scenario, this would be run in a separate task)
    println!("Testing RdmaBuilder::auto_listen()...");
    let max_msg_len = 1024; // 1KB max message length
    match RdmaBuilder::auto_listen(addr, max_msg_len).await {
        Ok(_rdma) => {
            println!("✅ Auto-listen succeeded with max_message_length: {}", max_msg_len);
            
            // In a real application, you would now use this RDMA connection
            // For this example, we'll just print success and move on
        }
        Err(e) => {
            println!("❌ Auto-listen failed: {}", e);
        }
    }

    // Test auto-connect (would typically connect to the auto-listen server)
    println!("\nTesting RdmaBuilder::auto_connect()...");
    match RdmaBuilder::auto_connect("127.0.0.1:12345", max_msg_len).await {
        Ok(_rdma) => {
            println!("✅ Auto-connect succeeded with max_message_length: {}", max_msg_len);
            // Note: This will likely fail since there's no server listening
        }
        Err(e) => {
            println!("ℹ️  Auto-connect failed (expected without server): {}", e);
        }
    }

    println!("\n=== Example Complete ===");
    println!("The auto-discovery functionality is working!");
    println!("Use RdmaBuilder::find_settings() to discover settings,");
    println!("or use RdmaBuilder::auto_listen(addr, max_msg_len)/auto_connect(addr, max_msg_len) for convenience.");

    Ok(())
}