interface Repository {
  name: string;
  description: string | null;
  topics: string[];
  homepage: string | null;
  language: string | null;
  url: string;
  stars: number;
  forks: number;
  issues: number;
}
