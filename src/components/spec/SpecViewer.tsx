import Markdown from "react-markdown";

interface Props {
  content: string;
}

export function SpecViewer({ content }: Props) {
  return (
    <div className="prose prose-invert prose-sm max-w-none">
      <Markdown
        components={{
          h1: ({ children }) => <h1 className="text-xl font-bold text-text mt-6 mb-3">{children}</h1>,
          h2: ({ children }) => <h2 className="text-lg font-semibold text-text mt-5 mb-2">{children}</h2>,
          h3: ({ children }) => <h3 className="text-base font-medium text-text mt-4 mb-2">{children}</h3>,
          p: ({ children }) => <p className="text-sm text-text-muted mb-2">{children}</p>,
          ul: ({ children }) => <ul className="list-disc list-inside text-sm text-text-muted mb-2 space-y-1">{children}</ul>,
          ol: ({ children }) => <ol className="list-decimal list-inside text-sm text-text-muted mb-2 space-y-1">{children}</ol>,
          li: ({ children }) => <li className="text-sm text-text-muted">{children}</li>,
          code: ({ children, className }) => {
            const isBlock = className?.includes("language-");
            if (isBlock) {
              return (
                <pre className="bg-surface border border-border rounded-lg p-3 overflow-x-auto my-2">
                  <code className="text-xs text-text">{children}</code>
                </pre>
              );
            }
            return (
              <code className="bg-surface px-1 py-0.5 rounded text-xs text-primary-light">{children}</code>
            );
          },
          strong: ({ children }) => <strong className="text-text font-semibold">{children}</strong>,
          a: ({ children, href }) => (
            <a href={href} className="text-primary-light hover:underline" target="_blank" rel="noreferrer">
              {children}
            </a>
          ),
        }}
      >
        {content}
      </Markdown>
    </div>
  );
}
