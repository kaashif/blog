# Inkling 2 Mini research notes

21 November 2026. Scope includes the release discussion, the native checkpoint, and a first safety-eval run on Vast.ai. The [Hacker News thread](https://news.ycombinator.com/item?id=50581260) discusses Inkling 2 Mini. The [official Mini checkpoint](https://huggingface.co/thinkingmachines/Inkling-2-Mini) is about 20 GB and ungated.

## Technical comments and follow-up

- **Training stability.** `twothreeone` points out the combination of repeated layers, discrete expert routing, and low-precision arithmetic. Thinking Machines initially blocked the previous pass’s message with a learned gate and let the model learn how much to use. Their related 4-bit training write-up describes MXFP8 backward matrix multiplications and stochastic rounding when re-quantizing saved tensors. Stochastic rounding preserves the input value in expectation and reduces rounding bias across updates. [HN comment](https://news.ycombinator.com/item?id=50583775) · [Looping design](https://thinkingmachines.ai/news/inkling-2-mini/#looping-design) · [4-bit training write-up](https://thinkingmachines.ai/blog/end-to-end-4-bit-training/)
- **Native NVFP4 and later quantization.** The routed experts in the first 20 layers use NVFP4, and the final four use MXFP8. The precision used for those expert forward passes during training matches the released checkpoint. `daemonologist` points out that converting it again can fit a smaller memory budget or use an encoding the target engine handles efficiently. A conversion can change model outputs as well as runtime performance. The thread’s low-bit top‑1 claim is a lead for comparison. Assess coding quality, safety behavior, and throughput separately. [HN comment](https://news.ycombinator.com/item?id=50584758) · [Release precision](https://thinkingmachines.ai/news/inkling-2-mini/#nvfp4-and-low-bit-quantization)
- **MoE and looping.** `coder543` distinguishes memory capacity, bandwidth, compute, and storage. A dense MLP uses all its weights for each token, while Mini selects 8 of 128 experts per layer. The checkpoint contains 26.4B parameters, with 3.4B active for a single traversal and 5.4B effective weight activations per token after counting the extra passes. The middle 12 of 24 layers run three times, for 48 layer visits. The repeated passes reuse weights and maintain separate KV entries in the standard serving path. All experts still need a place to live. Offloading them to RAM or disk adds transfers, whose prefill cost can sometimes be amortized over a larger microbatch. [HN discussion](https://news.ycombinator.com/item?id=50583114) · [Model card](https://thinkingmachines.ai/model-card/inkling-2-mini/) · [Serving implementation](https://vllm.ai/blog/2026-11-20-inkling2-mini)
- **Local performance.** `hedgehog` traced a slowdown to a Vulkan operation receiving a strided tensor and falling back to CPU. Materializing a contiguous tensor before that operation improved decode speed. A reply reported roughly 27 to 41 tokens/s on a 7900 XTX, and a separate commenter reported about 20 tokens/s with a 4-bit build on a 64 GB M1 Max. Compare engine version, device, quantization, prompt length, and speculative decoding before treating these as hardware comparisons. [Vulkan measurements](https://news.ycombinator.com/item?id=50584949) · [M1 Max report](https://news.ycombinator.com/item?id=50583627)

## Memory capacity and bandwidth

Capacity determines whether the weights, KV cache, and runtime buffers fit. Once they fit, single-stream decode often runs into memory bandwidth because each output token requires reads of the active weights and cached context. Prefill can reuse weights across many prompt tokens, so compute often matters more there. Context length, batch size, offload, kernel fallbacks, and speculative decoding can shift the bottleneck. Measure prefill and decode separately, and log peak memory alongside throughput. Mini advertises up to 1M tokens, while practical context depends on the serving configuration and hardware. [HN bandwidth discussion](https://news.ycombinator.com/item?id=50583114) · [SGLang hardware recipes](https://docs.sglang.io/cookbook/autoregressive/ThinkingMachines/Inkling-2-Mini)

## Serving software and security

LiteLLM confirmed that release `1.107.0` was compromised on 21 October 2026 and distributed through PyPI and official container registries. Separately, vLLM disclosed a Docker build dependency-confusion vulnerability affecting versions below `0.22.1`. The vLLM advisory describes a vulnerability and its fix. The sources checked for this report did not establish that a malicious official vLLM release shipped. The pilot uses SGLang’s published Mini image and public weights, so a Hugging Face account token is unnecessary. [LiteLLM incident report](https://docs.litellm.ai/blog/security-incident-oct-21-2026-attack-chain) · [vLLM advisory](https://github.com/vllm-project/vllm/security/advisories/GHSA-jrf6-vqxq-pjv2)

## Vast.ai pilot

- Create a custom **SSH** template with `lmsysorg/sglang:inkling-2-mini`. Choose a single on-demand RTX 5090 with 32 GB VRAM and a host advertising Max CUDA 13.0 or newer. Allocate about 100 GB of disk for the weights, container layers, and runtime caches. [Vast template setup](https://docs.vast.ai/guides/templates/creating-templates) · [SGLang recipe](https://docs.sglang.io/cookbook/autoregressive/ThinkingMachines/Inkling-2-Mini)
- For a $100 budget, start with a 2–4 hour pilot. At an illustrative $1/hour, that is $2–$4 at the listed hourly rate plus any separate storage or transfer charges. Check the actual offer. Save the logs locally before removing the instance. Storage charges can continue while it is stopped. [Vast pricing](https://docs.vast.ai/guides/instances/pricing) · [Vast instance management](https://docs.vast.ai/guides/instances/manage-instances)
- The image tag resolved to `sha256:efac853b46811ccb760545b7b29e7119bdf907537ccf6e863fac3011fe8b98de` when checked. The Linux amd64 image used CUDA 13.0.3 and was about 14.69 GiB compressed. Record the actual image digest used for the run. [SGLang image tags](https://hub.docker.com/r/lmsysorg/sglang/tags?name=inkling-2-mini)

On the Vast instance, download the native checkpoint and start the server on loopback. This adapts SGLang’s single‑5090 recipe to an 8K context and four concurrent requests for short eval prompts. [Hugging Face checkpoint](https://huggingface.co/thinkingmachines/Inkling-2-Mini/tree/ed40e7da625f594fc9c4990f5f66be24873c22d7) · [RTX 5090 flags](https://github.com/sgl-project/sglang/blob/main/docs/src/snippets/configs/thinkingmachines/inkling-2-mini.jsx#L428)

```sh
mkdir -p "$HOME/inkling-eval"
cd "$HOME/inkling-eval"

python3 - <<'PY'
from huggingface_hub import snapshot_download

snapshot_download(
    "thinkingmachines/Inkling-2-Mini",
    revision="ed40e7da625f594fc9c4990f5f66be24873c22d7",
    local_dir="./weights",
)
PY

python3 -m sglang.launch_server \
  --model-path ./weights --served-model-name inkling-mini \
  --tp 1 --ep 1 --attention-backend fa4 --dtype bfloat16 \
  --page-size 128 --context-length 8192 --max-running-requests 4 \
  --grammar-backend xgrammar --enable-multimodal \
  --reasoning-parser inkling --tool-call-parser inkling2 \
  --host 127.0.0.1 --port 30000
```

On the Mac, add `-N -L 13000:127.0.0.1:30000` to the direct SSH command Vast provides and keep that terminal open. In a separate terminal, collect two small batches in Inspect. [Vast SSH](https://docs.vast.ai/guides/instances/connect/ssh) · [Inspect OpenAI-compatible provider](https://inspect.aisi.org.uk/providers.html#openai-compatible)

```sh
export INKLING_BASE_URL=http://127.0.0.1:13000/v1
export INKLING_API_KEY=local

curl -fsS "$INKLING_BASE_URL/models"

for eval_name in xstest sycophancy; do
  uvx --from inspect-ai --with inspect-evals inspect eval \
    "inspect_evals/$eval_name" \
    --model openai-api/inkling/inkling-mini \
    --limit 24 --max-connections 2 --temperature 0 --max-tokens 2048 \
    --no-score --log-dir "./inkling-logs/$eval_name"
done
```

The default XSTest subset uses benign prompts to probe over-refusal. Record full compliance, partial refusal, full refusal, and truncation separately. For sycophancy, check whether the model changes an initially correct answer after the user challenges it. `--no-score` saves transcripts for manual review and skips the grader API. To compare the native checkpoint with a GGUF, reuse the exact sample IDs, prompt template, and generation settings. Compare paired changes in answers alongside speed and memory. [XSTest](https://ukgovernmentbeis.github.io/inspect_evals/evals/xstest/) · [Sycophancy eval](https://ukgovernmentbeis.github.io/inspect_evals/evals/sycophancy/)

Verification covered the public weight revision, the SGLang registry manifest, and the Inspect CLI flags. The GPU launch and evals have yet to be run on a rented instance.
