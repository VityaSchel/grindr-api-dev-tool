<script lang="ts">
	import { resolveRef, schemaTypeLabel, type SchemaObject } from "$lib/openapi";
	import * as Select from "$lib/components/ui/select";
	import * as RadioGroup from "$lib/components/ui/radio-group";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import { Input } from "$lib/components/ui/input";
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Switch } from "$lib/components/ui/switch";
	import { Toggle } from "$lib/components/ui/toggle";
	import { Button } from "$lib/components/ui/button";
	import Markdown from "$lib/components/Markdown.svelte";
	import TrashIcon from "phosphor-svelte/lib/TrashIcon";
	import PlusIcon from "phosphor-svelte/lib/PlusIcon";
	import SchemaField from "./SchemaField.svelte";

	type Props = {
		schema: SchemaObject;
		value: unknown;
		setValue: (v: unknown) => void;
		/** Field's own description (markdown). Falls back to the schema's own description. */
		description?: string;
		label?: string;
		required?: boolean;
		/** Whether this field is currently included in the parent (key present). */
		present?: boolean;
		/** Toggle inclusion in the parent; absent ⇒ not togglable (root / array item). */
		setPresent?: (b: boolean) => void;
		depth?: number;
	};

	type Kind =
		| "enum"
		| "boolean"
		| "number"
		| "string"
		| "object"
		| "array"
		| "oneOf";

	/** Resolve `$ref` and flatten `allOf` into a single inspectable schema. */
	function normalizeSchema(s: SchemaObject): SchemaObject {
		let cur = s;
		if (cur.$ref) cur = (resolveRef(cur.$ref) as SchemaObject) ?? {};
		if (cur.allOf?.length) {
			const merged: SchemaObject = {
				type: "object",
				properties: {},
				required: [],
			};
			for (const sub of cur.allOf) {
				const ns = normalizeSchema(sub);
				Object.assign(merged.properties!, ns.properties ?? {});
				merged.required = [...(merged.required ?? []), ...(ns.required ?? [])];
				if (ns.type) merged.type = ns.type;
				if (ns.description && !merged.description)
					merged.description = ns.description;
			}
			Object.assign(merged.properties!, cur.properties ?? {});
			return merged;
		}
		return cur;
	}

	/** The name of a referenced type (e.g. "Geohash"), if the raw schema is a ref. */
	function refName(s: SchemaObject): string | undefined {
		if (s.$ref) return s.$ref.split("/").pop();
		if (s.allOf?.length === 1 && s.allOf[0].$ref)
			return s.allOf[0].$ref.split("/").pop();
		return undefined;
	}

	function detectKind(s: SchemaObject): Kind {
		if (s.oneOf?.length || s.anyOf?.length) return "oneOf";
		if (s.enum?.length) return "enum";
		if (s.type === "boolean") return "boolean";
		if (s.type === "integer" || s.type === "number") return "number";
		if (s.type === "array") return "array";
		if (s.type === "object" || s.properties) return "object";
		return "string";
	}

	function isNullable(s: SchemaObject): boolean {
		return s.nullable === true || normalizeSchema(s).nullable === true;
	}

	/** Type-appropriate empty value, ignoring nullability. */
	function typedBlank(s: SchemaObject | undefined): unknown {
		if (!s) return "";
		const n = normalizeSchema(s);
		if (n.oneOf || n.anyOf) return undefined;
		if (n.type === "object" || n.properties) return {};
		if (n.type === "array") return [];
		if (n.type === "boolean") return false;
		if (n.type === "integer" || n.type === "number") return 0;
		return "";
	}

	/**
	 * Default for a freshly-materialised field. An empty string/zero still carries
	 * meaning to the API, so nullable fields start out as the more neutral `null`.
	 * (Optional fields aren't materialised here at all — only required ones are.)
	 */
	function blankItem(s: SchemaObject | undefined): unknown {
		if (s && isNullable(s)) return null;
		return typedBlank(s);
	}

	let {
		schema,
		value,
		setValue,
		description = undefined,
		label = undefined,
		required = false,
		present = true,
		setPresent = undefined,
		depth = 0,
	}: Props = $props();

	const resolved = $derived(normalizeSchema(schema));
	const kind = $derived(detectKind(resolved));
	const nullable = $derived(isNullable(schema));
	const isNull = $derived(value === null);
	const disabled = $derived(!present);
	const fieldId = $derived(
		`f-${label ?? "field"}-${depth}-${Math.random().toString(36).slice(2, 7)}`,
	);

	// Field's own description vs. the referenced type's description (hover).
	const fieldDescription = $derived(description ?? schema.description);
	const typeName = $derived(
		refName(schema) ?? schemaTypeLabel(schema) ?? schemaTypeLabel(resolved),
	);
	const typeDescription = $derived(
		refName(schema) ? resolved.description : undefined,
	);
	const showStar = $derived(required && !nullable);

	const objValue = $derived(
		value !== null && typeof value === "object" && !Array.isArray(value)
			? (value as Record<string, unknown>)
			: undefined,
	);
	const arrValue = $derived(
		Array.isArray(value) ? (value as unknown[]) : undefined,
	);

	// Materialise containers (only while present) so nested fields can write.
	// A deliberate `null` is left intact (see the NULL toggle) — and a nullable
	// container defaults to that neutral `null` rather than an empty `{}`/`[]`, which
	// would still carry meaning to the API. Only non-nullable containers get a blank.
	$effect(() => {
		if (!present || isNull) return;
		if (kind === "object" && !objValue) setValue(nullable ? null : {});
		else if (kind === "array" && !arrValue) setValue(nullable ? null : []);
	});

	const objProps = $derived(Object.entries(resolved.properties ?? {}));
	const requiredSet = $derived(new Set(resolved.required ?? []));
	const itemSchema = $derived(resolved.items);

	// Required properties are always present.
	$effect(() => {
		if (!present || !objValue) return;
		for (const [name, sub] of objProps) {
			if (requiredSet.has(name) && !(name in objValue))
				objValue[name] = blankItem(sub);
		}
	});

	// ── enum helpers ──
	const enumOptions = $derived(
		(resolved.enum ?? []).map((e) => ({
			v: String(e),
			original: e,
			label: resolved["x-enum-labels"]?.[String(e)] ?? String(e),
		})),
	);
	const currentStr = $derived(
		value === undefined || value === null ? "" : String(value),
	);
	function edit(v: unknown) {
		setValue(v);
	}
	function pickEnum(v: string | undefined) {
		const opt = enumOptions.find((o) => o.v === v);
		edit(opt ? opt.original : undefined);
	}
	const useRadio = $derived(enumOptions.length > 0 && enumOptions.length <= 4);

	// ── oneOf/anyOf ──
	const variants = $derived(resolved.oneOf ?? resolved.anyOf ?? []);
	let variantIdx = $state(0);

	function onString(e: Event & { currentTarget: HTMLInputElement }) {
		const v = e.currentTarget.value;
		edit(v === "" ? undefined : v);
	}
	function onNumber(e: Event & { currentTarget: HTMLInputElement }) {
		const v = e.currentTarget.value;
		if (v === "") return edit(undefined);
		const n = Number(v);
		edit(Number.isNaN(n) ? undefined : n);
	}
	function setNull(p: boolean) {
		// Un-nulling must yield an editable value, not `null` again.
		edit(p ? null : typedBlank(resolved));
	}
</script>

{#snippet typeBadge()}
	{#if typeName && typeDescription}
		<Tooltip.Provider delayDuration={150}>
			<Tooltip.Root>
				<Tooltip.Trigger
					class="cursor-help font-mono text-[0.7rem] text-muted-foreground underline decoration-dotted underline-offset-2"
				>
					{typeName}
				</Tooltip.Trigger>
				<Tooltip.Content class="max-w-xs">
					<Markdown text={typeDescription} />
				</Tooltip.Content>
			</Tooltip.Root>
		</Tooltip.Provider>
	{:else if typeName}
		<span class="font-mono text-[0.7rem] text-muted-foreground">{typeName}</span
		>
	{/if}
{/snippet}

{#snippet fieldHeader()}
	{#if label !== undefined}
		<div class="flex flex-col gap-1">
			<div class="flex items-center gap-2">
				{#if setPresent}
					<Checkbox
						checked={present}
						disabled={required}
						onCheckedChange={(c) => setPresent?.(c === true)}
						class="size-3.5"
						aria-label="Include {label}"
					/>
				{/if}
				<span
					class={[
						"text-xs font-medium",
						resolved.deprecated && "text-muted-foreground line-through",
						!present && "text-muted-foreground",
					]}
				>
					{label}{#if showStar}<span class="text-destructive">*</span>{/if}
				</span>
				{@render typeBadge()}
			</div>
			{#if fieldDescription}
				<Markdown text={fieldDescription} />
			{/if}
		</div>
	{/if}
{/snippet}

{#snippet nullToggle()}
	{#if nullable}
		<Toggle
			size="sm"
			variant="outline"
			pressed={isNull}
			{disabled}
			onPressedChange={setNull}
			class={[
				"h-8 shrink-0 px-1.5 font-mono text-[0.65rem] tracking-wide",
				{
					"bg-primary hover:bg-primary-foreground": isNull,
				},
			]}
			aria-label="Set null"
		>
			NULL
		</Toggle>
	{/if}
{/snippet}

{#if kind === "object"}
	<div class="flex flex-col gap-2">
		{#if label !== undefined || nullable}
			<div class="flex items-start justify-between gap-1.5">
				<div class="min-w-0 flex-1">{@render fieldHeader()}</div>
				{@render nullToggle()}
			</div>
		{/if}
		{#if present && !isNull && objValue}
			{#if objProps.length}
				<!-- Only the properties are indented under the object, not its header. -->
				<div
					class={[
						"flex flex-col gap-3",
						depth > 0 && "border-l border-muted pl-3",
					]}
				>
					{#each objProps as [propName, propSchema] (propName)}
						<SchemaField
							schema={propSchema}
							label={propSchema["x-display-name"] ?? propName}
							required={requiredSet.has(propName)}
							present={propName in objValue}
							setPresent={(b) => {
								if (b) {
									if (!(propName in objValue))
										objValue[propName] = blankItem(propSchema);
								} else {
									delete objValue[propName];
								}
							}}
							depth={depth + 1}
							value={objValue[propName]}
							setValue={(v) => (objValue[propName] = v)}
						/>
					{/each}
				</div>
			{:else}
				<p class="text-xs text-muted-foreground italic">
					No defined properties (free-form object).
				</p>
			{/if}
		{/if}
	</div>
{:else if kind === "array"}
	<div class="flex flex-col gap-2">
		{#if label !== undefined || nullable}
			<div class="flex items-start justify-between gap-1.5">
				<div class="min-w-0 flex-1">{@render fieldHeader()}</div>
				{@render nullToggle()}
			</div>
		{/if}
		{#if present && !isNull}
			{#if arrValue && arrValue.length}
				<div class="flex flex-col gap-2 border-l border-muted pl-3">
					{#each arrValue as item, i (i)}
						<div class="flex items-start gap-2">
							<div class="flex-1">
								<SchemaField
									schema={itemSchema ?? { type: "string" }}
									label={`#${i}`}
									depth={depth + 1}
									value={item}
									setValue={(v) => (arrValue[i] = v)}
								/>
							</div>
							<Button
								type="button"
								variant="ghost"
								size="icon-sm"
								onclick={() => arrValue.splice(i, 1)}
								aria-label="Remove item"
							>
								<TrashIcon />
							</Button>
						</div>
					{/each}
				</div>
			{/if}
			<div>
				<Button
					type="button"
					variant="outline"
					size="xs"
					onclick={() => arrValue?.push(blankItem(itemSchema))}
				>
					<PlusIcon /> Add item
				</Button>
			</div>
		{/if}
	</div>
{:else if kind === "oneOf"}
	<div class="flex flex-col gap-2">
		{@render fieldHeader()}
		{#if present}
			<Select.Root
				type="single"
				value={String(variantIdx)}
				onValueChange={(v) => (variantIdx = Number(v))}
			>
				<Select.Trigger class="w-full">
					{schemaTypeLabel(normalizeSchema(variants[variantIdx] ?? {})) ||
						`Option ${variantIdx + 1}`}
				</Select.Trigger>
				<Select.Content>
					{#each variants as variant, i (i)}
						<Select.Item value={String(i)}>
							{schemaTypeLabel(normalizeSchema(variant)) || `Option ${i + 1}`}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
			{#if variants[variantIdx]}
				<SchemaField
					schema={variants[variantIdx]}
					{value}
					{setValue}
					depth={depth + 1}
				/>
			{/if}
		{/if}
	</div>
{:else if kind === "boolean"}
	<div class="flex flex-col gap-1.5">
		{@render fieldHeader()}
		<div class="flex items-center gap-2">
			{@render nullToggle()}
			<Switch
				checked={value === true}
				disabled={disabled || isNull}
				onCheckedChange={(c) => edit(c)}
			/>
		</div>
	</div>
{:else if kind === "enum"}
	<div class="flex flex-col gap-1.5">
		{@render fieldHeader()}
		<div class="flex items-center gap-1.5">
			{@render nullToggle()}
			<div class="flex-1">
				{#if useRadio}
					<RadioGroup.Root
						class="flex flex-wrap gap-x-4 gap-y-1.5 pt-0.5"
						value={currentStr}
						disabled={disabled || isNull}
						onValueChange={pickEnum}
					>
						{#each enumOptions as opt (opt.v)}
							<div class="flex items-center gap-1.5">
								<RadioGroup.Item value={opt.v} id={`${fieldId}-${opt.v}`} />
								<label for={`${fieldId}-${opt.v}`} class="text-xs"
									>{opt.label}</label
								>
							</div>
						{/each}
					</RadioGroup.Root>
				{:else}
					<Select.Root
						type="single"
						value={currentStr}
						disabled={disabled || isNull}
						onValueChange={pickEnum}
					>
						<Select.Trigger class="w-full">
							{enumOptions.find((o) => o.v === currentStr)?.label ??
								"Select..."}
						</Select.Trigger>
						<Select.Content>
							{#each enumOptions as opt (opt.v)}
								<Select.Item value={opt.v} label={opt.label}
									>{opt.label}</Select.Item
								>
							{/each}
						</Select.Content>
					</Select.Root>
				{/if}
			</div>
		</div>
	</div>
{:else if kind === "number"}
	<div class="flex flex-col gap-1.5">
		{@render fieldHeader()}
		<div class="flex items-center gap-1.5">
			{@render nullToggle()}
			<Input
				id={fieldId}
				type="number"
				step={resolved.type === "integer" ? "1" : "any"}
				value={isNull ? "" : currentStr}
				disabled={disabled || isNull}
				placeholder={isNull ? "null" : ""}
				oninput={onNumber}
				class="flex-1"
			/>
		</div>
	</div>
{:else}
	<div class="flex flex-col gap-1.5">
		{@render fieldHeader()}
		<div class="flex items-center gap-1.5">
			{@render nullToggle()}
			<Input
				id={fieldId}
				type="text"
				value={isNull ? "" : currentStr}
				disabled={disabled || isNull}
				placeholder={isNull ? "null" : ""}
				oninput={onString}
				class="flex-1"
			/>
		</div>
	</div>
{/if}
