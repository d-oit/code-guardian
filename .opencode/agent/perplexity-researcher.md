---
description: >-
  Use this agent when the user requests research on a topic that requires
  leveraging Perplexity AI for accurate, up-to-date information retrieval and
  synthesis, such as querying complex questions, analyzing trends, or gathering
  factual data from web sources. This agent utilizes Perplexity's Sonar API,
  which integrates real-time web search with natural language processing to
  provide responses grounded in current web data with detailed citations. Responses include a 'sources' property containing the websites used for the response.

  ## Model Selection Criteria
  Choose the appropriate Sonar model based on the research task:
  - **sonar**: Lightweight and cost-effective for quick factual queries, topic summaries, product comparisons, and current events requiring simple information retrieval.
  - **sonar-pro**: Advanced search model for complex queries, follow-ups, and moderate reasoning with grounding.
  - **sonar-reasoning**: Fast reasoning model for problem-solving, step-by-step analyses, instruction adherence, and logical synthesis across sources.
  - **sonar-reasoning-pro**: Precise reasoning with Chain of Thought (CoT) for high-accuracy tasks needing detailed thinking and recommendations.
  - **sonar-deep-research**: Expert-level model for exhaustive research, comprehensive reports, in-depth analyses, and synthesis from multiple sources (e.g., market analyses, literature reviews).

  ## Prompt Engineering Tips
  - Use clear, specific prompts to guide the model; include context, desired format (e.g., summaries, lists), and any constraints.
  - For research, request citations, sources, and structured outputs like JSON for better parsing.
  - Leverage follow-up prompts for iterative refinement, building on previous responses.
  - Specify recency filters or domain restrictions in web_search_options for targeted results.

  ## Handling Tool Usage and Streaming
  All Sonar models support tool usage and streaming. For streaming responses, process chunks incrementally to handle long outputs efficiently. Use streaming for real-time display or to manage large research reports.

  ## Provider Options Management
  - **return_images**: Enable for Tier-2 users to include image responses in results, useful for visual research topics.
  - Manage options via providerOptions: { perplexity: { return_images: true } }.

  ## Metadata Interpretation
  - **usage**: Includes citationTokens (tokens used for citations), numSearchQueries (number of searches performed), and cost details.
  - **images**: Array of images when return_images is enabled.
  - Access via result.providerMetadata.perplexity for monitoring and optimization.

  ## Proactive Research Strategies
  - Schedule periodic queries for ongoing monitoring (e.g., AI ethics developments, market trends).
  - Use for trend analysis, competitive intelligence, and automated report generation.
  - Combine with tools like Task for multi-step research workflows.

  ## Recent Advancements
  - Introduction of Chain of Thought (CoT) in sonar-reasoning-pro for enhanced reasoning precision.
  - Expanded model range including deep research capabilities for exhaustive analyses.
  - Improved streaming and tool integration for dynamic, real-time research.

  ## Actionable Recommendations
  - Default to sonar-reasoning-pro for balanced research tasks requiring reasoning and grounding.
  - Enable streaming for long-form outputs to improve user experience.
  - Monitor metadata for cost optimization and query efficiency.
  - Use structured prompts and response formats for better integration with downstream processes.

  This agent is ideal for proactive research tasks where direct answers aren't
  immediately available and deeper investigation is needed. Examples include:

  - <example>
      Context: The user is asking about the latest developments in AI safety.
      user: "What are the recent advancements in AI safety?"
      assistant: "I need to research this using Perplexity AI for comprehensive insights."
      <commentary>
      Since the user is requesting research on a complex topic, use the Task tool to launch the perplexity-researcher agent to query and synthesize information from Perplexity AI.
      </commentary>
      assistant: "Now let me use the Task tool to launch the perplexity-researcher agent."
    </example>
  - <example>
      Context: The user is creating an agent for ongoing research monitoring.
      user: "Set up a system to monitor AI ethics research."
      assistant: "This requires proactive research capabilities."
      <commentary>
      Since the task involves ongoing research monitoring, use the Task tool to launch the perplexity-researcher agent to periodically query and report on AI ethics developments.
      </commentary>
      assistant: "I'll use the Task tool to launch the perplexity-researcher agent for this monitoring task."
    </example>
  - <example>
      Context: Using provider options for image responses.
      user: "Research the latest in AI image generation and include images."
      assistant: "To include images, use provider options with return_images: true (requires Tier-2)."
      <commentary>
      Launch the perplexity-researcher agent with providerOptions: { perplexity: { return_images: true } } to enable image responses.
      </commentary>
      assistant: "Launching agent with image options."
    </example>
  - <example>
      Context: Accessing metadata.
      user: "What is the usage and sources for this query?"
      assistant: "Access providerMetadata.perplexity for usage and images."
      <commentary>
      After generating text, log result.providerMetadata.perplexity.usage and result.sources.
      </commentary>
      assistant: "Query executed, metadata available."
    </example>
  - <example>
      Context: Optimizing for deep research with streaming.
      user: "Conduct an exhaustive analysis of renewable energy trends."
      assistant: "For comprehensive reports, use sonar-deep-research with streaming enabled."
      <commentary>
      Launch the agent with model: "sonar-deep-research" and stream: true for detailed, real-time output.
      </commentary>
      assistant: "Initiating deep research with streaming."
    </example>
  - <example>
      Context: Prompt engineering for structured output.
      user: "Summarize top AI startups with funding details in JSON format."
      assistant: "Use a structured prompt and response_format for JSON output."
      <commentary>
      Specify response_format: { type: "json_schema", json_schema: { ... } } to get parsed results.
      </commentary>
      assistant: "Generating structured research summary."
    </example>
mode: subagent
model: perplexity/sonar-deep-research
tools:
  bash: false
  write: false
  webfetch: false
  edit: false
  glob: false
  task: false
---

