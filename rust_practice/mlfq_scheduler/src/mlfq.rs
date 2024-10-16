#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
        // TODO: Implement this function
        // Add the process to the appropriate queue based on its priority
        // Ensure the priority is within the valid range (0 to num_levels - 1)

        // If process priority out of range, append to lowest priority
        if (process.priority < 0) || (process.priority > self.num_levels - 1) {
            self.queues[self.num_levels - 1].push(process);
        }
        
        // Otherwise, append to process priority queue
        else {
            self.queues[process.priority].push(process);
        }

    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        // TODO: Implement this function
        // Execute the process for its time quantum or until completion
        // Update remaining_time, total_executed_time, and current_time
        // Move the process to a lower priority queue if it doesn't complete

        // Start timer
        let mut timer = 0;

        // Check for empty queue
        if self.queues[queue_index].len() > 0 {

            // Let process be the first one in queue
            let mut process = self.queues[queue_index][0].clone();

            // Run program while timer is less than quanta for queue
            while timer < self.time_quanta[queue_index] {
                process.total_executed_time += 1;
                process.remaining_time -= 1;

                timer += 1;
                self.current_time += 1;

                // Break if no remaining time
                if process.remaining_time == 0 {
                    break;
                }
            }

            // If process has time left, add to new queue
            if process.remaining_time != 0 {
                self.queues.remove(queue_index);
                
                // If already in lowest priority queue, readd
                if queue_index == self.num_levels - 1 {
                    self.queues[self.num_levels - 1].push(process);
                }

                // If not in lowest priority queue, lower priority by 1
                else {
                    process.priority += 1;
                    self.queues[queue_index + 1].push(process);
                }
            }

            // Otherwise, remove process from list
            else {
                self.queues.remove(queue_index);
            }
        }
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // TODO: Implement this function
        // Move all processes to the highest priority queue
        // Reset the priority of all processes to 0

        // Iterate over all queues
        for i in 1..(self.num_levels) {

            // Move ALL processes and update priority
            while self.queues[i].len() > 0 {
                let mut process = self.queues[i][0].clone();
                process.priority = 0;

                self.queues[0].push(process);
                self.queues[i].remove(0);
            }
        }
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    // Test for updated priority in run process (could include in previous one)
    #[test]
    fn test_execute_process_updated_priority() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[1][0].priority, 1);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    // Test for multiple processes in queue (could test in previous one)
    #[test]
    fn test_priority_boost_multiple_processes() {
        let mut mlfq = MLFQ::new(4, vec![2,4,8,16]);

        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[1].push(Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[1].push(Process { id: 3, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.queues[3].push(Process { id: 4, priority: 3, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 4);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
        assert_eq!(mlfq.queues[3].len(), 0);

    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}