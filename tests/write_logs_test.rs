#[cfg(test)]
mod tests {
    use chrono::Utc;

    use risk_rust::gamelogic::logs::{init_logger, write_log};
    use risk_rust::routing::GameLog;
    use std::fs;
    use std::thread;

    // Helper function to read the log file content
    fn read_log_file() -> String {
        fs::read_to_string("game.log").unwrap_or_else(|_| String::new())
    }

    // Test for successful logging of a few messages
    #[test]
    fn test_write_log_few_messages() {
        // Remove the log file if it exists from previous tests
        let _ = fs::remove_file("game.log");
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| init_logger());
        let game_log1 = GameLog {
            current_time: Utc::now(),
            username: "player1".to_string(),
            message: "Joined the game".to_string(),
        };
        let game_log2 = GameLog {
            current_time: Utc::now(),
            username: "player2".to_string(),
            message: "Started a new quest".to_string(),
        };

        assert!(write_log(game_log1).is_ok());
        assert!(write_log(game_log2).is_ok());

        let log_content = read_log_file();
        assert!(log_content.contains("player1:Joined the game"));
        assert!(log_content.contains("player2:Started a new quest"));
    }

    //Test for handling many messages (potential failure scenario)
    #[test]
    fn test_write_log_many_messages() {
        // Remove the log file if it exists from previous tests
        let _ = fs::remove_file("game.log");

        let base_game_log = GameLog {
            current_time: Utc::now(),
            username: "player".to_string(),
            message: "Logged a message".to_string(),
        };

        let num_threads = 10; // Simulate multiple users/threads logging simultaneously
        let messages_per_thread = 1000; // Number of messages each thread will log

        let mut handles = vec![];

        for i in 0..num_threads {
            let game_log = base_game_log.clone();
            let handle = thread::spawn(move || {
                for j in 0..messages_per_thread {
                    let current_log = GameLog {
                        current_time: Utc::now(),
                        username: format!("{}{}", game_log.username, i),
                        message: format!("{}{}", game_log.message, j),
                    };

                    let _ = write_log(current_log);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Check if the log file is significantly smaller than expected,
        // indicating potential message loss
        let log_content = read_log_file();
        let expected_lines = num_threads * messages_per_thread;
        let actual_lines = log_content.lines().count();
        // Allow for some margin of error.  It's possible some lines will be lost when many are written simultaneously.
        let tolerance = 0.8;

        println!(
            "Expected lines: {}, Actual lines: {}",
            expected_lines, actual_lines
        );
        assert!(
            (actual_lines as f64) >= (expected_lines as f64) * tolerance,
            "Significant message loss detected. Expected at least {} lines, but found only {} lines.",
            (expected_lines as f64) * tolerance,
            actual_lines
        );
    }
}
