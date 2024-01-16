# Projection

Easily scaffold new projects from custom templates

## Usage

Any directory under `<HOMEDIR>/projection` will automatically be interpreted as a template.

```txt
<HOMEDIR>/
└── projection/
    ├── template1
    ├── template2
    └── ...
```

### Initialising a template

To initialise a template in your current directory, run `pj create <TEMPLATE_NAME>`.

Optionally, you can specify a destination folder: `pj create <TEMPLATE_NAME> [DESTINATION]`

### Listing available templates

You can list all detected templates via the `pj list` command.

```txt
├── template1
├── template2
└── test_template
```
