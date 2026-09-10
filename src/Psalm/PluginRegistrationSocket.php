<?php

declare(strict_types=1);

namespace Psalm;

use Override;
use Psalm\Plugin\EventHandler\DynamicFunctionStorageProviderInterface;
use Psalm\Plugin\EventHandler\FunctionExistenceProviderInterface;
use Psalm\Plugin\EventHandler\FunctionParamsProviderInterface;
use Psalm\Plugin\EventHandler\FunctionReturnTypeProviderInterface;
use Psalm\Plugin\EventHandler\MethodExistenceProviderInterface;
use Psalm\Plugin\EventHandler\MethodParamsProviderInterface;
use Psalm\Plugin\EventHandler\MethodReturnTypeProviderInterface;
use Psalm\Plugin\EventHandler\MethodVisibilityProviderInterface;
use Psalm\Plugin\EventHandler\PropertyExistenceProviderInterface;
use Psalm\Plugin\EventHandler\PropertyTypeProviderInterface;
use Psalm\Plugin\EventHandler\PropertyVisibilityProviderInterface;
use Psalm\Plugin\RegistrationInterface;


final class PluginRegistrationSocket implements RegistrationInterface
{
    /**
     * @internal
     * @psalm-mutation-free
     */
    public function __construct(
        public readonly Config $config,
        public readonly Codebase $codebase,
    ) {
    }

    /**
     * @psalm-external-mutation-free
     */
    #[Override]
    public function addStubFile(string $file_name): void
    {
        $this->config->addStubFile($file_name);
    }

    #[Override]
    public function registerHooksFromClass(object $handler): void
    {
        $this->config->eventDispatcher->registerClass($handler);

        if ($handler instanceof PropertyExistenceProviderInterface) {
            $this->codebase->properties->property_existence_provider->registerClass($handler);
        }

        if ($handler instanceof PropertyVisibilityProviderInterface) {
            $this->codebase->properties->property_visibility_provider->registerClass($handler);
        }

        if ($handler instanceof PropertyTypeProviderInterface) {
            $this->codebase->properties->property_type_provider->registerClass($handler);
        }

        if ($handler instanceof MethodExistenceProviderInterface) {
            $this->codebase->methods->existence_provider->registerClass($handler);
        }

        if ($handler instanceof MethodVisibilityProviderInterface) {
            $this->codebase->methods->visibility_provider->registerClass($handler);
        }

        if ($handler instanceof MethodReturnTypeProviderInterface) {
            $this->codebase->methods->return_type_provider->registerClass($handler);
        }

        if ($handler instanceof MethodParamsProviderInterface) {
            $this->codebase->methods->params_provider->registerClass($handler);
        }

        if ($handler instanceof FunctionExistenceProviderInterface) {
            $this->codebase->functions->existence_provider->registerClass($handler);
        }

        if ($handler instanceof FunctionParamsProviderInterface) {
            $this->codebase->functions->params_provider->registerClass($handler);
        }

        if ($handler instanceof FunctionReturnTypeProviderInterface) {
            $this->codebase->functions->return_type_provider->registerClass($handler);
        }

        if ($handler instanceof DynamicFunctionStorageProviderInterface) {
            $this->codebase->functions->dynamic_storage_provider->registerClass($handler);
        }
    }
}
