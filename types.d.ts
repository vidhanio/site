interface Post {
  slug: string;
  code: string;
  frontmatter: {
    title: string;
    description: string;
    imageURL: string | null;
    dateAdded: [number, number, number];
    dateEdited: [number, number, number] | null;
  };
}

interface FrontmatterProps {
  title: string;
  description: string;
  imageURL?: string;
  dateAdded: Date;
  dateEdited?: Date;
}

interface SEOProps {
  title: string;
  description: string;
  imageURL: string | null;
  slug: string;
}
