fn main() {
    let data = get_data();
    let mut job_pairs = Vec::new();
    for input in data.lines() {
        job_pairs.push(JobPair::new(input));
    }

    let mut total_overlap = 0;
    for pair in &job_pairs {
        if pair.is_fully_overlap {
            total_overlap += 1;
        }
    }

    println!("Question 1 answer: {}", total_overlap);

    total_overlap = 0;
    for a_pair in job_pairs {
        if a_pair.is_overlap_at_all {
            total_overlap += 1;
        }
    }

    println!("Question 2 answer: {}", total_overlap);
}

fn get_data() -> String {
    return std::fs::read_to_string("day4/src/data.txt").unwrap_or_default();
}

#[derive(Debug)]
struct Job {
    start: u32,
    end: u32,
}

impl Job {
    fn new(data: &str) -> Self {
        let mut pieces = data.split('-');
        let start: u32 = pieces
            .next()
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let end: u32 = pieces
            .next()
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        Self { start, end }
    }

    fn fully_overlaps(&self, job: &Job) -> bool {
        if self.start <= job.start && self.end >= job.end {
            return true;
        }

        if self.end <= job.start && self.end >= job.end {
            return true;
        }

        false
    }
    fn overlaps_at_all(&self, job: &Job) -> bool {
        let mut range = Vec::new();
        for n in self.start..=self.end {
            range.push(n);
        }

        for n in job.start..=job.end {
            if range.contains(&n) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
struct JobPair {
    is_fully_overlap: bool,
    is_overlap_at_all: bool,
}

impl JobPair {
    fn new(pair_data: &str) -> Self {
        let mut pairs = pair_data.split(',');
        let job1 = Job::new(pairs.next().unwrap_or_default());
        let job2 = Job::new(pairs.next().unwrap_or_default());
        let is_overlap_at_all = job1.overlaps_at_all(&job2);
        let mut is_fully_overlap = job1.fully_overlaps(&job2);
        if !is_fully_overlap {
            is_fully_overlap = job2.fully_overlaps(&job1)
        }

        Self {
            is_fully_overlap,
            is_overlap_at_all,
        }
    }
}
