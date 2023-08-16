# {{ creatorly.project_name }} Documentation

This project is generated from a template!

## Empty {{ }}

## Variable doesn't exists {{ creatorly.test }}

## Exclude some text not be rendered

- {% raw %}${{ if startsWith(variables['build.sourceBranch'], 'refs/tags/') }}{% endraw %}
- {% raw %}${{ env.{% endraw %}{{ creatorly.project_name }} {% raw %}}}{% endraw %}
