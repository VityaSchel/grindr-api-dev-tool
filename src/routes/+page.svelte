<script lang="ts">
	import HttpMethod from "$lib/components/HttpMethod.svelte";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import * as Tabs from "$lib/components/ui/tabs";
	import * as Card from "$lib/components/ui/card";
	import { Label } from "$lib/components/ui/label";

	let url = $state("");

	const baseUrl = "https://grindr.mobi/";

	let tab = $state("vars");
</script>

<main>
	<div class="flex gap-2">
		<HttpMethod />
		<Input
			bind:value={
				() => baseUrl + url,
				(v) => {
					url = v.startsWith(baseUrl) ? v.replace(baseUrl, "") : v;
				}
			}
		/>
		<Button>Send</Button>
	</div>
	<div class="flex w-full max-w-sm flex-col gap-6">
		<Tabs.Root bind:value={tab}>
			<Tabs.List>
				<Tabs.Trigger value="account">Account</Tabs.Trigger>
				<Tabs.Trigger value="password">Password</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="account">
				<Card.Root>
					<Card.Header>
						<Card.Title>Account</Card.Title>
						<Card.Description>
							Make changes to your account here. Click save when you&apos;re
							done.
						</Card.Description>
					</Card.Header>
					<Card.Content class="grid gap-6">
						<div class="grid gap-3">
							<Label for="tabs-demo-name">Name</Label>
							<Input id="tabs-demo-name" value="Pedro Duarte" />
						</div>
						<div class="grid gap-3">
							<Label for="tabs-demo-username">Username</Label>
							<Input id="tabs-demo-username" value="@peduarte" />
						</div>
					</Card.Content>
					<Card.Footer>
						<Button>Save changes</Button>
					</Card.Footer>
				</Card.Root>
			</Tabs.Content>
			<Tabs.Content value="password">
				<Card.Root>
					<Card.Header>
						<Card.Title>Password</Card.Title>
						<Card.Description>
							Change your password here. After saving, you&apos;ll be logged
							out.
						</Card.Description>
					</Card.Header>
					<Card.Content class="grid gap-6">
						<div class="grid gap-3">
							<Label for="tabs-demo-current">Current password</Label>
							<Input id="tabs-demo-current" type="password" />
						</div>
						<div class="grid gap-3">
							<Label for="tabs-demo-new">New password</Label>
							<Input id="tabs-demo-new" type="password" />
						</div>
					</Card.Content>
					<Card.Footer>
						<Button>Save password</Button>
					</Card.Footer>
				</Card.Root>
			</Tabs.Content>
		</Tabs.Root>
	</div>
</main>
