#!/usr/bin/env bash
# Adopt — instala o AI Dev Workflow em um projeto JÁ EXISTENTE.
#
# Diferenças vs bootstrap.sh:
#   - Não assume projeto vazio; preserva tudo que já existe.
#   - Detecta stack automaticamente (package.json, pyproject.toml, app.json, etc.).
#   - Cria apenas o esqueleto MÍNIMO de docs/ (PROGRESS.md, decisions-log.md, adr/).
#   - Não toca em README.md, .gitignore, .env.example existentes.
#   - Sai com instruções para o agente popular PROGRESS.md a partir do código real.
#
# Uso:
#   cd /caminho/para/projeto-existente
#   bash /caminho/para/ai-dev-workflow/scripts/adopt.sh
#
# Flags:
#   --stack X          Força stack (nextjs|node-backend|python|mobile|none).
#   --name NAME        Nome do projeto (default: basename do diretório).
#   --dry-run          Mostra o que faria, sem escrever nada.
#   --minimal          Instala só o mínimo (workflow.instructions.md + PROGRESS.md + AGENTS.md).
#   --yes              Não pergunta confirmação.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKFLOW_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
TARGET_DIR="$(pwd)"

STACK=""
PROJECT_NAME=""
DRY_RUN=false
MINIMAL=false
ASSUME_YES=false
WORKFLOW_VERSION="$(tr -d '[:space:]' < "$WORKFLOW_DIR/VERSION" 2>/dev/null || echo "0.0.0")"
INSTALLED_VERSION=""
[[ -f "$TARGET_DIR/.aidw-version" ]] && INSTALLED_VERSION="$(tr -d '[:space:]' < "$TARGET_DIR/.aidw-version")"

major_minor() {
  printf '%s' "$1" | awk -F. '{ print $1 "." $2 }'
}

is_significant_update() {
  [[ -n "$1" && "$(major_minor "$1")" != "$(major_minor "$2")" ]]
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --stack) STACK="$2"; shift 2 ;;
    --name) PROJECT_NAME="$2"; shift 2 ;;
    --dry-run) DRY_RUN=true; shift ;;
    --minimal) MINIMAL=true; shift ;;
    --yes|-y) ASSUME_YES=true; shift ;;
    -h|--help)
      sed -n '2,22p' "$0" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "Flag desconhecida: $1" >&2; exit 1 ;;
  esac
done

if [[ "$TARGET_DIR" == "$WORKFLOW_DIR" ]]; then
  echo "ERRO: rodando dentro do próprio repositório do workflow." >&2
  exit 1
fi

# --- Helpers ---
say()   { echo "→ $*"; }
warn()  { echo "⚠️  $*"; }
do_run() {
  if [[ "$DRY_RUN" == true ]]; then
    echo "   [dry-run] $*"
  else
    eval "$@"
  fi
}

# --- Detecção de stack ---
detect_stack() {
  if [[ -f "$TARGET_DIR/next.config.js" || -f "$TARGET_DIR/next.config.mjs" || -f "$TARGET_DIR/next.config.ts" ]]; then
    echo "nextjs"; return
  fi
  if [[ -f "$TARGET_DIR/app.json" ]] && grep -q '"expo"' "$TARGET_DIR/app.json" 2>/dev/null; then
    echo "mobile"; return
  fi
  if [[ -f "$TARGET_DIR/pyproject.toml" || -f "$TARGET_DIR/requirements.txt" ]]; then
    echo "python"; return
  fi
  if [[ -f "$TARGET_DIR/package.json" ]]; then
    if grep -qE '"(fastify|express|@nestjs/core)"' "$TARGET_DIR/package.json" 2>/dev/null; then
      echo "node-backend"; return
    fi
    if grep -q '"next"' "$TARGET_DIR/package.json" 2>/dev/null; then
      echo "nextjs"; return
    fi
    echo "node-backend"; return
  fi
  echo "none"
}

# --- Inventário rápido (apenas exibe) ---
inventory() {
  echo
  echo "📋 Inventário do projeto:"
  echo "   Diretório: $TARGET_DIR"
  [[ -f "$TARGET_DIR/package.json" ]] && echo "   • package.json detectado"
  [[ -f "$TARGET_DIR/pyproject.toml" ]] && echo "   • pyproject.toml detectado"
  [[ -f "$TARGET_DIR/Cargo.toml" ]] && echo "   • Cargo.toml detectado"
  [[ -f "$TARGET_DIR/go.mod" ]] && echo "   • go.mod detectado"
  [[ -f "$TARGET_DIR/app.json" ]] && echo "   • app.json (Expo?) detectado"
  [[ -d "$TARGET_DIR/.git" ]] && echo "   • repositório git: sim"
  [[ -d "$TARGET_DIR/docs" ]] && echo "   • docs/ já existe (será preservado)"
  [[ -f "$TARGET_DIR/README.md" ]] && echo "   • README.md já existe (não será tocado)"
  [[ -d "$TARGET_DIR/.github" ]] && echo "   • .github/ já existe (será MESCLADO)"
  echo
}

inventory

say "AI Dev Workflow version: $WORKFLOW_VERSION"
if [[ -n "$INSTALLED_VERSION" ]]; then
  say "Installed version: $INSTALLED_VERSION"
  if is_significant_update "$INSTALLED_VERSION" "$WORKFLOW_VERSION"; then
    warn "Update significativo detectado: $INSTALLED_VERSION → $WORKFLOW_VERSION"
  fi
fi

# --- Stack ---
if [[ -z "$STACK" ]]; then
  detected="$(detect_stack)"
  say "Stack detectada: $detected"
  if [[ "$ASSUME_YES" == false ]]; then
    read -r -p "Confirma '$detected'? [Y/n] (ou digite outra: nextjs|node-backend|python|mobile|none) " ans
    case "${ans:-Y}" in
      [Yy]|"") STACK="$detected" ;;
      nextjs|node-backend|python|mobile|none) STACK="$ans" ;;
      [Nn]) echo "Abortado."; exit 0 ;;
      *) echo "Opção inválida"; exit 1 ;;
    esac
  else
    STACK="$detected"
  fi
fi
say "Stack escolhida: $STACK"

# --- Nome ---
if [[ -z "$PROJECT_NAME" ]]; then
  PROJECT_NAME="$(basename "$TARGET_DIR")"
fi
say "Nome do projeto: $PROJECT_NAME"

if [[ "$ASSUME_YES" == false && "$DRY_RUN" == false ]]; then
  echo
  read -r -p "Adotar AI Dev Workflow em $TARGET_DIR? [y/N] " ans
  [[ "$ans" =~ ^[Yy]$ ]] || { echo "Abortado."; exit 0; }
fi

SED_INPLACE=( -i )
case "$(uname)" in
  Darwin) SED_INPLACE=( -i '' ) ;;
esac

# --- 1. Copia .github/ (sem sobrescrever existentes) ---
say "Copiando .github/ (preservando arquivos do projeto)..."
if [[ "$DRY_RUN" == false ]]; then
  mkdir -p "$TARGET_DIR/.github"
  # -n = no clobber: nunca sobrescreve.
  cp -Rn "$WORKFLOW_DIR/.github/." "$TARGET_DIR/.github/" 2>/dev/null || true
fi

# Filtra stacks
if [[ "$STACK" != "none" ]]; then
  STACKS_DIR="$TARGET_DIR/.github/instructions/stacks"
  if [[ -d "$STACKS_DIR" ]]; then
    for f in "$STACKS_DIR"/*.instructions.md; do
      [[ -e "$f" ]] || continue
      base="$(basename "$f" .instructions.md)"
      if [[ "$base" != "$STACK" && "$base" != "README" ]]; then
        do_run "rm -f \"$f\""
      fi
    done
  fi
fi

# --- Modo MINIMAL: descarta tudo exceto workflow.instructions.md ---
if [[ "$MINIMAL" == true ]]; then
  say "Modo --minimal: removendo instruções não-essenciais..."
  INST_DIR="$TARGET_DIR/.github/instructions"
  if [[ -d "$INST_DIR" ]]; then
    for f in "$INST_DIR"/*.instructions.md; do
      [[ -e "$f" ]] || continue
      base="$(basename "$f")"
      if [[ "$base" != "workflow.instructions.md" ]]; then
        do_run "rm -f \"$f\""
      fi
    done
  fi
  do_run "rm -rf \"$TARGET_DIR/.github/prompts\""
  do_run "rm -rf \"$TARGET_DIR/.github/chatmodes\""
fi

# --- 2. AGENTS.md / CLAUDE.md (não sobrescreve) ---
for f in AGENTS.md CLAUDE.md; do
  if [[ -f "$TARGET_DIR/$f" ]]; then
    warn "$f já existe — preservado. Mescle manualmente se quiser."
  else
    say "Criando $f..."
    do_run "cp \"$WORKFLOW_DIR/$f\" \"$TARGET_DIR/$f\""
  fi
done

# --- 2.1. Agent skills (atualiza skills do workflow) ---
if [[ -d "$WORKFLOW_DIR/skills" ]]; then
  say "Atualizando skills agnosticas de agente..."
  if [[ "$DRY_RUN" == false ]]; then
    mkdir -p "$TARGET_DIR/skills"
    cp -R "$WORKFLOW_DIR/skills/." "$TARGET_DIR/skills/" 2>/dev/null || true
  fi
fi

# --- 2.2. Version marker ---
if [[ "$DRY_RUN" == false ]]; then
  echo "$WORKFLOW_VERSION" > "$TARGET_DIR/.aidw-version"
  say "Gravada versão instalada em .aidw-version"
fi

# --- 3. docs/ — só o mínimo, preservando o que existe ---
say "Criando esqueleto mínimo de docs/ (sem tocar arquivos existentes)..."
do_run "mkdir -p \"$TARGET_DIR/docs/progress\" \"$TARGET_DIR/docs/adr\""

# PROGRESS.md
if [[ -f "$TARGET_DIR/docs/progress/PROGRESS.md" ]]; then
  warn "docs/progress/PROGRESS.md já existe — preservado."
else
  do_run "cp \"$WORKFLOW_DIR/templates/docs/progress/PROGRESS.md\" \"$TARGET_DIR/docs/progress/PROGRESS.md\""
fi

# decisions-log
if [[ -f "$TARGET_DIR/docs/progress/decisions-log.md" ]]; then
  warn "docs/progress/decisions-log.md já existe — preservado."
else
  do_run "cp \"$WORKFLOW_DIR/templates/docs/progress/decisions-log.md\" \"$TARGET_DIR/docs/progress/decisions-log.md\""
fi

# ADR template (não cria 0001 — vai ser retroativa)
if [[ ! -f "$TARGET_DIR/docs/adr/0000-template.md" ]]; then
  do_run "cp \"$WORKFLOW_DIR/templates/docs/adr/0000-template.md\" \"$TARGET_DIR/docs/adr/0000-template.md\""
fi

# Estrutura adicional só se NÃO for minimal
if [[ "$MINIMAL" == false ]]; then
  for sub in architecture features risks postmortem spikes user-stories; do
    if [[ ! -d "$TARGET_DIR/docs/$sub" ]]; then
      do_run "mkdir -p \"$TARGET_DIR/docs/$sub\""
      # copia templates apenas se houver
      if [[ -d "$WORKFLOW_DIR/templates/docs/$sub" ]]; then
        # Preserva qualquer coisa que já tenha sido criada
        for src in "$WORKFLOW_DIR/templates/docs/$sub"/*; do
          [[ -e "$src" ]] || continue
          name="$(basename "$src")"
          dst="$TARGET_DIR/docs/$sub/$name"
          if [[ ! -e "$dst" ]]; then
            do_run "cp \"$src\" \"$dst\""
          fi
        done
      fi
    fi
  done
fi

# --- 4. Substitui placeholders nos arquivos NOVOS apenas ---
if [[ "$DRY_RUN" == false ]]; then
  say "Substituindo {{PROJECT_NAME}} → $PROJECT_NAME (apenas em arquivos novos)..."
  # Como copiamos com -n, só os novos têm placeholder não-substituído.
  find "$TARGET_DIR/docs" -type f -name '*.md' -exec sed "${SED_INPLACE[@]}" "s/{{PROJECT_NAME}}/${PROJECT_NAME}/g" {} \; 2>/dev/null || true
  find "$TARGET_DIR/.github" -type f -name '*.md' -exec sed "${SED_INPLACE[@]}" "s/{{PROJECT_NAME}}/${PROJECT_NAME}/g" {} \; 2>/dev/null || true
fi

# --- 5. .env.example: só cria se NÃO existir ---
if [[ ! -f "$TARGET_DIR/.env.example" ]]; then
  say "Criando .env.example mínimo..."
  if [[ "$DRY_RUN" == false ]]; then
    cat > "$TARGET_DIR/.env.example" <<EOF
# Variáveis de ambiente — copie para .env.local e preencha.
# NUNCA comite .env.local com valores reais.
EOF
  fi
fi

# --- Resumo final ---
echo
echo "✅ Adoção concluída$( [[ "$DRY_RUN" == true ]] && echo " (dry-run — nada foi escrito)")."
echo "Versão instalada: $WORKFLOW_VERSION"
echo
echo "Próximos passos (faça com o agente):"
echo "  1. Abra o projeto no seu editor com Copilot/Claude/Codex."
echo "  2. Comece pela skill 'atlas' no seu agente e peça para adotar o projeto existente."
echo "     - Ela vai rotear para adopt-existing-project, inventariar o código, popular PROGRESS.md, e propor ADRs retroativas."
echo "  3. Revise o que o agente gerou em:"
echo "       - docs/progress/PROGRESS.md"
echo "       - docs/adr/0001-stack-inicial.md (se gerada)"
echo "       - docs/architecture/tech-stack.md"
echo "  4. Commit: git add -A && git commit -m \"chore: adopt ai-dev-workflow\""
echo
echo "Dica: comece a usar o fluxo completo só nas PRÓXIMAS features. Não tente"
echo "      retrofittar o passado todo de uma vez."
echo
