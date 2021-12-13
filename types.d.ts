interface Post {
  title: string;
  description: string;
  imageURL: string | null;
  slug: string;
  content: string;
  dateAdded: string;
  dateUpdated: string | null;
}

interface FrontmatterProps {
  title: string;
  description: string;
  imageURL?: string;
}
